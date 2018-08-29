use game::map::tile::*;
use tcod::bsp::*;

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
        //if node.is_leaf() {
            box_draw(&mut ret, node.x, node.y, node.w, node.h);
        //}
        return true;
    });
    return ret;
}