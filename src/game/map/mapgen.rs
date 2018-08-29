extern crate rand;
use game::map::tile::Tile;
use game::map::tile::Wall;
use tcod::bsp::*;
use self::rand::{thread_rng, Rng};
use std::cmp::{min, max};

pub fn dummy_gen() -> Vec<Box<Tile>> {
    let mut ret: Vec<Box<Tile>> = Vec::new();
    ret.push(Box::new(Wall::new(20, 20)));

    return ret;
}

fn box_draw(map: &mut Vec<Box<Tile>>, x: i32, y: i32, w: i32, h: i32) {
    
    //map.set_default_foreground(tcod::colors::Color::new(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()));
    for i in (x)..(x + w){
        /*
        root.put_char(i , y + 1, 205 as u8 as char, BackgroundFlag::None);
        root.put_char(i, y + h - 1, 205 as u8 as char, BackgroundFlag::None);
        */
        map.push(Box::new(Wall::new(i, y )));
        map.push(Box::new(Wall::new(i, y + h)));
    }
    for i in (y)..(y + h) {
        /*
        root.put_char(x + 1, i, 186 as u8 as char, BackgroundFlag::None);
        root.put_char(x + w - 1, i, 186 as u8 as char, BackgroundFlag::None);
        */
        map.push(Box::new(Wall::new(x, i)));
        map.push(Box::new(Wall::new(x + w , i)));
    }
    map.push(Box::new(Wall::new(x + w, y + h)));
    /*
    root.put_char(x + w - 1, y + h - 1, /*'╝'*/ 188 as u8 as char, BackgroundFlag::None);
    root.put_char(x + 1, y + 1,/*'╔'*/201 as u8 as char, BackgroundFlag::None);
    root.put_char(x + w - 1, y + 1, /*'╗'*/187 as u8 as char, BackgroundFlag::None);
    root.put_char(x + 1, y + h - 1, /*'╚'*/200 as u8 as char, BackgroundFlag::None);
    root.flush();*/
    //═║╔╗╚╝
}

pub fn bsp_gen(recursion_levels:     i32, 
               min_horizontal_size:  i32, 
               min_vertical_size:    i32,
               max_horizontal_ratio: f32,
               max_vertical_ratio:   f32
    ) -> Vec<Box<Tile>> {
    let mut ret: Vec<Box<Tile>> = Vec::new();
    let mut bsp = Bsp::new_with_size(0, 0, 79, 49);
    bsp.split_recursive(Option::None, recursion_levels, 
                                      min_horizontal_size, 
                                      min_vertical_size, 
                                      max_horizontal_ratio, 
                                      max_vertical_ratio);
    bsp.traverse(TraverseOrder::LevelOrder, |node| {
        let mut rng = thread_rng();
        if node.is_leaf() {
            let point1: (i32, i32) = (rng.gen_range(node.x + 1, node.x + node.w),rng.gen_range(node.y + 1, node.y + node.h));
            let point2: (i32, i32) = (rng.gen_range(node.x + 1, node.x + node.w),rng.gen_range(node.y + 1, node.y + node.h));
            
            box_draw(&mut ret, point1.0, point1.1, point2.0, point2.1);
            /*
            let x = rng.gen_range(node.x + 1, node.x + node.w);
            let y = rng.gen_range(node.y + 1, node.y + node.h);
            let mut tempvec: Vec<Box<Tile>> = Vec::new();
            let root: Box = Box::new(Wall::new(x, y));
            tempvec.push(root);
            let mut temp: Box = Box::new(Wall::new(-1, -1));
            while *root.get_x() != *temp.get_x() || *root.get_y() != *temp.get_y() {
                if !ret.contains(Box::new(::new(8temp.get_x()), *temp.get_y()) && !tempvec.contains(Tile::new(temp.get_x(), temp.get_y()) {
                    println!("hello");
                }
            }
            println!("{} {}", node.x, node.x + node.w);
            let mut rng = thread_rng();
            //let x1 =0;
            //let x2 =5;
            //let y1 =0;
            //let y2 =5;
            let x1 = rng.gen_range(node.x, ((node.x + node.w ) / 2) as i32);
            let x2 = rng.gen_range(x1, node.x + node.w) + 1;
            let y1 = rng.gen_range(node.y, ((node.y + node.h ) / 2) as i32);
            box_draw(&mut ret, x1, y1, x2, y2);
            */
        }
        return true;
    });
    return ret;
}
