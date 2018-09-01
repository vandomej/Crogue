use tcod::Map;
use game::map::tile::*;
use tcod::bsp::*;
use tcod::random::*;

fn create_tile(w: i32, h: i32, map_width: i32, map_height: i32) -> Box<Tile> {
    if w == 0 || w == map_width-1 || h == 0 || h == map_height-1 || (w % 3 == 1 && h % 2 == 1) {
        Box::new(Wall::new(w, h))
    } else {
        Box::new(Floor::new(w, h))
    }
}

fn create_tile2(w: i32, h: i32, map_width: i32, map_height: i32) -> Box<Tile> {
    if w == 0 || w == map_width-1 || h == 0 || h == map_height-1 {
        Box::new(Wall::new(w, h))
    } else {
        Box::new(Floor::new(w, h))
    }
}

pub fn dummy_gen(map_width: i32, map_height: i32) -> (Map, Vec<Box<Tile>>) {
    let mut tiles: Vec<Box<Tile>> = Vec::new();
    let mut map = Map::new(map_width, map_height);

    for h in 0..map_height {
        for w in 0..map_width {
            let tile = create_tile(w, h, map_width, map_height);

            map.set(w, h, tile.get_see_through(), tile.get_walkable());
            tiles.push(tile);
        }
    }

    return (map, tiles);
}

pub fn empty_gen(map_width: i32, map_height: i32) -> (Map, Vec<Box<Tile>>) {
    let mut tiles: Vec<Box<Tile>> = Vec::new();
    let mut map = Map::new(map_width, map_height);

    for h in 0..map_height {
        for w in 0..map_width {
            let tile = create_tile2(w, h, map_width, map_height);

            map.set(w, h, tile.get_see_through(), tile.get_walkable());
            tiles.push(tile);
        }
    }

    return (map, tiles);
}

fn box_draw(map: &mut Map, tiles: &mut Vec<Box<Tile>>, x: i32, y: i32, w: i32, h: i32, rng: &Rng, frame: bool, min_area: i32) {
    if w * h < min_area ||
        w     < 6        ||
            h     < 6 { 
                return;
            }
    if frame {
        for i in x..(x+w) {
            //map[y as usize][i as usize] = Box::new(Wall::new(i, y));
            //map[(y + h) as usize][i as usize] = Box::new(Wall::new(i, y + h));
            let tile = Box::new(Wall::new(i, y));
            let tile2 = Box::new(Wall::new(i, y + h));
            map.set(i, y, tile.get_see_through(), tile.get_walkable());
            map.set(i, y + h, tile2.get_see_through(), tile2.get_walkable());
            tiles.push(tile);
            tiles.push(tile2);
        }
        for i in y..(y+h) {
            //map[i as usize][x as usize] = Box::new(Wall::new(x, i));
            //map[i as usize][(x + w) as usize] = Box::new(Wall::new(x + w, i));
            let tile = Box::new(Wall::new(x, i));
            let tile2 = Box::new(Wall::new(x + w, i));
            map.set(x, i, tile.get_see_through(), tile.get_walkable());
            map.set(x + w, i, tile2.get_see_through(), tile2.get_walkable());
            tiles.push(tile);
            tiles.push(tile2);
        }
    }
    let x_half = x + (w / 2) + 1;
    let y_half = y + (h / 2) + 1;
    let mut x1 = rng.get_int(x + 2, x_half);
    let mut y1 = rng.get_int(y + 2, y_half);
    let mut x2 = rng.get_int(x_half + 1, x + w - 2);
    let mut y2 = rng.get_int(y_half + 1, y + h - 2);
    while (x2 - x1) * (y2 - y1) < min_area {
        if x1 - x > 2         { x1 -= 1; }
        if x + w - 2 - x2 > 2 { x2 += 1; }
        if y1 - y > 2         { y1 -= 1; }
        if y + h - 2 - y2 > 2 { y2 += 1; }
        if x1 - x <= 2         &&
            x + w - 2 - x2 <= 2 &&
                y1 - y <= 2         &&
                y + h - 2 - y2 <= 2 {
                    return;
                }
    }
    for i in x1..x2 {
        if i != x1 + 2 {
            let tile = Box::new(Wall::new(i, y1));
            let tile2 = Box::new(Wall::new(i, y2));
            map.set(i, y1, tile.get_see_through(), tile.get_walkable());
            map.set(i, y2, tile2.get_see_through(), tile2.get_walkable());
            tiles.push(tile);
            tiles.push(tile2);
        }
    }
    for i in y1..y2 {
        let tile = Box::new(Wall::new(x1, i));
        let tile2 = Box::new(Wall::new(x2, i));
        map.set(x1, i, tile.get_see_through(), tile.get_walkable());
        map.set(x2, i, tile2.get_see_through(), tile2.get_walkable());
        tiles.push(tile);
        tiles.push(tile2);
    }
    let tile = Box::new(Wall::new(x2, y2));
    map.set(x2, y2, tile.get_see_through(), tile.get_walkable());
    /*
       '╗' 187
       '╝' 188
       '╔' 201
       '╚' 200
       '║' 205
       '═' 186
    //═║╔╗╚╝
    */
}

pub fn bsp_gen(recursion_levels:     i32, 
               min_horizontal_size:  i32, 
               min_vertical_size:    i32,
               max_horizontal_ratio: f32,
               max_vertical_ratio:   f32,
               seed:                 u32,
               min_area:             i32,
               frame:                bool
              ) -> (Map, Vec<Box<Tile>>) {
    let mut ret: (Map, Vec<Box<Tile>>) = empty_gen(80, 50);
    let mut bsp = Bsp::new_with_size(0, 0, 79, 49);
    let rng = Rng::new_with_seed(Algo::MT, seed);
    bsp.split_recursive(Some(rng), recursion_levels, 
                        min_horizontal_size, 
                        min_vertical_size, 
                        max_horizontal_ratio, 
                        max_vertical_ratio);
    let rng = Rng::new_with_seed(Algo::MT, seed);
    bsp.traverse(TraverseOrder::LevelOrder, |node| {
        if node.is_leaf() {
            box_draw(&mut ret.0, &mut ret.1, node.x, node.y, node.w, node.h, &rng, frame, min_area);
        }
        return true;
    });
    return ret;
}
