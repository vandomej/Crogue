use tcod::Map;
use game::map::tile::*;
use tcod::bsp::*;
use tcod::random::*;
use config::*;

fn create_tile(w: i32, h: i32, map_width: i32, map_height: i32) -> Box<Tile> {
    if w == 0 || w == map_width-1 || h == 0 || h == map_height-1 || (w % 3 == 1 && h % 2 == 1) {
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
            new_floor(&mut map, &mut tiles, w, h);
        }
    }
    draw_box(&mut map, &mut tiles, 0, 0, map_width - 1, map_height - 1);

    return (map, tiles);
}

fn new_floor(map: &mut Map, tiles: &mut Vec<Box<Tile>>, x: i32, y: i32) {
    let tile = Box::new(Floor::new(x, y));
    map.set(x, y, tile.get_see_through(), tile.get_walkable());
    tiles.push(tile);
}

fn new_wall(map: &mut Map, tiles: &mut Vec<Box<Tile>>, x: i32, y: i32) {
    let tile = Box::new(Wall::new(x, y));
    let see_through = if CONFIG.game.see_all { true } else { tile.get_see_through() };
    map.set(x, y, see_through, tile.get_walkable());
    tiles.push(tile);
}

fn draw_box(map: &mut Map, tiles: &mut Vec<Box<Tile>>, x: i32, y: i32, w: i32, h: i32) {
    for i in x..w {
        new_wall(map, tiles, i, y);
        new_wall(map, tiles, i, h);
    }
    for i in y..h {
        new_wall(map, tiles, x, i);
        new_wall(map, tiles, w, i);
    }
    new_wall(&mut *map, &mut *tiles, w, h);
}

fn single_room(map: &mut Map, tiles: &mut Vec<Box<Tile>>, x: i32, y: i32, w: i32, h: i32, rng: &Rng, frame: bool, min_area: i32) {
    if w * h < min_area {
       return;
    }   
    if frame {
        draw_box(map, tiles, x, y, x + w, y + h);
    }
    let x_half = x + (w / 2) + 1;
    let y_half = y + (h / 2) + 1;
    let mut x1 = rng.get_int(x + 2, x_half);
    let mut y1 = rng.get_int(y + 2, y_half);
    let mut x2 = rng.get_int(x_half + 1, x + w - 2);
    let mut y2 = rng.get_int(y_half + 1, y + h - 2);
    let x1_test = | x1 | { x1    - 2 - x  > 0 };
    let y1_test = | y1 | { y1    - 2 - y  > 0 };
    let x2_test = | x2 | { x + w - 2 - x2 > 0 };
    let y2_test = | y2 | { y + h - 2 - y2 > 0 };
    while (x2 - x1) * (y2 - y1) < min_area {
        if x1_test(x1) { x1 -= 1; }
        if x2_test(x2) { x2 += 1; }
        if y1_test(y1) { y1 -= 1; }
        if y2_test(y2) { y2 += 1; }
        if !x1_test(x1) && !x2_test(x2) && !y1_test(y1) && !y2_test(y2) {
            return;
        }
    }
    draw_box(map, tiles, x1, y1, x2, y2);
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

pub fn bsp_gen() -> (Map, Vec<Box<Tile>>) {
    let mut ret: (Map, Vec<Box<Tile>>) = empty_gen(CONFIG.game.screen_width, CONFIG.game.screen_height);
    let mut bsp = Bsp::new_with_size(0, 0, CONFIG.game.screen_width - 1, CONFIG.game.screen_height - 1);
    let rng = Rng::new_with_seed(Algo::MT, CONFIG.bsp.seed);
    bsp.split_recursive(Some(rng), CONFIG.bsp.recursion_levels, 
                        CONFIG.bsp.min_horizontal_size, 
                        CONFIG.bsp.min_vertical_size, 
                        CONFIG.bsp.max_horizontal_ratio, 
                        CONFIG.bsp.max_vertical_ratio);
    let rng = Rng::new_with_seed(Algo::MT, CONFIG.bsp.seed);
    bsp.traverse(TraverseOrder::LevelOrder, |node| {
        if node.is_leaf() {
            single_room(&mut ret.0, &mut ret.1, node.x, node.y, node.w, node.h, &rng, CONFIG.bsp.frame, CONFIG.bsp.min_area);
        }
        return true;
    });
    return ret;
}
