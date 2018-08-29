use game::map::tile::*;

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
