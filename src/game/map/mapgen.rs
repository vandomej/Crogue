use tcod::Map;
use game::map::tile::*;

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
