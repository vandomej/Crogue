extern crate rand;
use game::map::tile::*;
use tcod::bsp::*;
use tcod::random::*;
use std::cmp::{max, min};

fn create_tile(w: i32, h: i32, map_width: i32, map_height: i32) -> Box<Tile> {
    if w == 0 || w == map_width-1 || h == 0 || h == map_height-1 {
        Box::new(Wall::new(w, h))
    } else {
        Box::new(Floor::new(w, h))
    }
}

pub fn dummy_gen(map_width: i32, map_height: i32) -> Vec<Vec<Box<Tile>>> {
    let mut container: Vec<Vec<Box<Tile>>> = Vec::new();

    for h in 0..map_height {
        let mut row: Vec<Box<Tile>> = Vec::new();

        for w in 0..map_width {
            let entry = create_tile(w, h, map_width, map_height);

            row.push(entry);
        }

        container.push(row);
    }

    return container;
}

fn box_draw(map: &mut Vec<Vec<Box<Tile>>>, x: i32, y: i32, w: i32, h: i32) {
    
        //map.set_default_foreground(tcod::colors::Color::new(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()));
        /*
        root.put_char(i , y + 1, 205 as u8 as char, BackgroundFlag::None);
        root.put_char(i, y + h - 1, 205 as u8 as char, BackgroundFlag::None);
        root.put_char(x + 1, i, 186 as u8 as char, BackgroundFlag::None);
        root.put_char(x + w - 1, i, 186 as u8 as char, BackgroundFlag::None);
        root.put_char(x + w - 1, y + h - 1, /*'╝'*/ 188 as u8 as char, BackgroundFlag::None);
        root.put_char(x + 1, y + 1,/*'╔'*/201 as u8 as char, BackgroundFlag::None);
        root.put_char(x + w - 1, y + 1, /*'╗'*/187 as u8 as char, BackgroundFlag::None);
        root.put_char(x + 1, y + h - 1, /*'╚'*/200 as u8 as char, BackgroundFlag::None);
        root.flush();
        //═║╔╗╚╝
        */
    //let mut rng = thread_rng();
    let rng = Rng::new_with_seed(Algo::MT, 1337);
    let mut x1 = rng.get_int(x, x + w + 1);
    let mut x2 = rng.get_int(min(x1, x + w), max(x1, x + w) + 1);
    let mut y1 = rng.get_int(y, y + h + 1);
    let mut y2 = rng.get_int(min(y1, y + h), max(y1, y + h) + 1);
    let min_area = 30;
    if w * h  <= min_area {
        return;
    }
    let mut counter = 10;
    while !(x1 > x && x2 < x + w && 
            y1 > y && y2 < y + h &&
           (x2 - x1) * (y2 - y1) >= min_area) {
        if counter == 0 { return; } else { counter -= 1; }
        x1 = rng.get_int(x, x + w + 1);
        x2 = rng.get_int(min(x1, x + w), max(x1, x + w) + 1);
        y1 = rng.get_int(y, y + h + 1);
        y2 = rng.get_int(min(y1, y + h), max(y1, y + h) + 1);
    }
    for i in x1..x2 {
        map[y1 as usize][i as usize] = Box::new(Wall::new(i, y1));
        map[y2 as usize][i as usize] = Box::new(Wall::new(i, y2));
    }
    for i in y1..y2 {
        map[i as usize][x1 as usize] = Box::new(Wall::new(x1, i));
        map[i as usize][x2 as usize] = Box::new(Wall::new(x2, i));
    }
    map[y2 as usize][x2 as usize] = Box::new(Wall::new(x2, y2));
}

pub fn bsp_gen(recursion_levels:     i32, 
               min_horizontal_size:  i32, 
               min_vertical_size:    i32,
               max_horizontal_ratio: f32,
               max_vertical_ratio:   f32
    ) -> Vec<Vec<Box<Tile>>> {
    let mut ret: Vec<Vec<Box<Tile>>> = dummy_gen(80, 50);
    let mut bsp = Bsp::new_with_size(0, 0, 79, 49);
    bsp.split_recursive(Some(Rng::new_with_seed(Algo::MT, 1337)), recursion_levels, 
                                      min_horizontal_size, 
                                      min_vertical_size, 
                                      max_horizontal_ratio, 
                                      max_vertical_ratio);
    bsp.traverse(TraverseOrder::LevelOrder, |node| {
        if node.is_leaf() {
            box_draw(&mut ret, node.x, node.y, node.w, node.h);
        }
        //ret[node.y as usize][node.x as usize] = Box::new(Wall::new(node.x, node.y));
        return true;
    });
    return ret;
}
