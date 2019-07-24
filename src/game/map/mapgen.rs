use tcod::Map;
use game::map::tile::*;
use tcod::bsp::*;
use tcod::random::*;
use tcod::line::Line;
use config::*;

/*
struct Map {
    rooms: Vec<room>,
    bsp_frames: Vec<room>,
}
*/

pub struct Room {
    xy: (i32, i32),
    wh: (i32, i32),
    walls: Vec<Line>
}

fn create_tile((w, h): (i32, i32), (map_width, map_height): (i32, i32)) -> Box<Tile> {
    if w == 0 || w == map_width-1 || h == 0 || h == map_height-1 || (w % 3 == 1 && h % 2 == 1) {
        Box::new(Wall::new((w, h)))
    } else {
        Box::new(Floor::new((w, h)))
    }
}

pub fn dummy_gen((map_w, map_h): (i32, i32)) -> (Map, Vec<Box<Tile>>) {
    let mut tiles: Vec<Box<Tile>> = Vec::new();
    let mut map = Map::new(map_w, map_h);

    for h in 0..map_w {
        for w in 0..map_h {
            let tile = create_tile((w, h), (map_w, map_h));

            map.set(w, h, tile.get_see_through(), tile.get_walkable());
            tiles.push(tile);
        }
    }

    return (map, tiles);
}

pub fn empty_gen((map_width, map_height): (i32, i32)) -> (Map, Vec<Box<Tile>>) {
    let mut tiles: Vec<Box<Tile>> = Vec::new();
    let mut map = Map::new(map_width, map_height);

    for h in 0..map_height {
        for w in 0..map_width {
            new_floor(&mut map, &mut tiles, (w, h));
        }
    }
    draw_box(&mut map, &mut tiles, (0, 0), (map_width - 1, map_height - 1));

    return (map, tiles);
}

fn new_floor(map: &mut Map, tiles: &mut Vec<Box<Tile>>, xy: (i32, i32)) {
    let tile = Box::new(Floor::new(xy));
    map.set(xy.0, xy.1, tile.get_see_through(), tile.get_walkable());
    tiles.push(tile);
}

fn new_wall(map: &mut Map, tiles: &mut Vec<Box<Tile>>, xy: (i32, i32)) {
    let tile = Box::new(Wall::new(xy));
    let see_through = if CONFIG.game.see_all { true } else { tile.get_see_through() };
    map.set(xy.0, xy.1, see_through, tile.get_walkable());
    tiles.push(tile);
}

fn draw_box(map: &mut Map, tiles: &mut Vec<Box<Tile>>, (x, y): (i32, i32), (w, h): (i32, i32)) {
    for i in x..w {
        new_wall(map, tiles, (i, y));
        new_wall(map, tiles, (i, h));
    }
    for i in y..h {
        new_wall(map, tiles, (x, i));
        new_wall(map, tiles, (w, i));
    }
    new_wall(&mut *map, &mut *tiles, (w, h));
}

fn gen_room(room: &Room, rng: &Rng, frame: bool, min_area: i32) -> Option<Room> {
    let (x, y) = room.xy;
    let (w, h) = room.wh;

    if w * h < min_area {
       return None;
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
            return None;
        }
    }

    /*
    room.walls.push(Line::new((x1,y1),(x1,y2)));
    room.walls.push(Line::new((x1,y1),(x2,y1)));
    room.walls.push(Line::new((x1,y2),(x2,y2)));
    room.walls.push(Line::new((x2,y1),(x2,y2)));
    */

    /*
       '╗' 187
       '╝' 188
       '╔' 201
       '╚' 200
       '║' 205
       '═' 186
    //═║╔╗╚╝
    */
    return Some(Room {xy: (x1, y1), wh: (x2 - x1, y2 - y1), walls: Vec::new()});
}

pub fn get_walls((x, y): (i32, i32), (w, h): (i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
    vec![
        ((x,y),(x + w, y)),
        ((x,y),(x, y + h)),
        ((x + w, y),(x + w, y + h)),
        ((x, y + h),(x + w, y + h))
    ]
}

fn within_another_room(wall: (i32, i32), rooms: &Vec<Room>) -> bool {
    for room in rooms {
        if wall.0 > room.xy.0 && wall.0 < (room.xy.0 + room.wh.0) &&
           wall.1 > room.xy.1 && wall.1 < (room.xy.1 + room.wh.1) {
            return true;
        }
    }

    false
}

pub fn add_to_map(rooms: Vec<Room>, m: Map, t: Vec<Box<Tile>>) -> (Map, Vec<Box<Tile>>){
    let mut map = m;
    let mut tiles = t;

    for room in &rooms {

        if !within_another_room(room.xy, &rooms) {
            new_wall(&mut map, &mut tiles, room.xy);
        }

        for wall in get_walls(room.xy, room.wh) {
            let mut line = Line::new(wall.0, wall.1);

            while let Some(w) = line.step() {
                if !within_another_room(w, &rooms) {
                    new_wall(&mut map, &mut tiles, w)
                }
            }
        }
    }
    return (map, tiles)
}

fn generate_connecting_hallway(r1: &Room, r2: &Room) -> (Room, Room) {
    let center_r1: (i32, i32) = (r1.xy.0 + r1.wh.0/2, r1.xy.1 + r1.wh.1/2);
    let center_r2: (i32, i32) = (r2.xy.0 + r2.wh.0/2, r2.xy.1 + r2.wh.1/2);
    let center_delta: (i32, i32) = (center_r2.0 - center_r1.0, center_r2.1 - center_r1.1);

    let hallway_horiz_xy = if center_delta.0 >= 0 {
        (center_r1.0, center_r2.1)
    } else {
        (center_r2.0, center_r2.1)
    };

    let hallway_vert_xy = if center_delta.1 >= 0 {
        (center_r1.0, center_r1.1)
    } else {
        (center_r1.0, center_r2.1)
    };

    let hallway_horizontal = Room {
        xy: hallway_horiz_xy,
        wh: (center_delta.0.abs() + 1, 2),
        walls: Vec::new()
    };

    let hallway_vertical = Room {
        xy: hallway_vert_xy,
        wh: (2, center_delta.1.abs() + 1),
        walls: Vec::new()
    };

    return (hallway_horizontal, hallway_vertical)
}

pub fn generate_rooms(frames: &Vec<Room>) -> Vec<Room> {
    let rng = Rng::new_with_seed(Algo::MT, CONFIG.bsp.seed);
    let mut rooms: Vec<Room> = Vec::new();
    let mut hallways: Vec<Room> = Vec::new();

    let mut prev_room = 0;
    let mut curr_room = 1;
    let mut rooms_len = 0;

    for frame in frames {
        if let Some(r) = gen_room(frame, &rng, CONFIG.bsp.frame, CONFIG.bsp.min_area) {

            rooms.push(r);
            rooms_len += 1;

            if rooms_len >= 2 {
                let hallway = generate_connecting_hallway(&rooms[prev_room], &rooms[curr_room]);

                hallways.push(hallway.0);
                hallways.push(hallway.1);

                prev_room += 1;
                curr_room += 1;
            }
        }
    };

    rooms.append(&mut hallways);
    return rooms;
}

pub fn generate_frames() -> Vec<Room> {
    let mut bsp = Bsp::new_with_size(0, 0, CONFIG.game.screen_width - 1, CONFIG.game.screen_height - 1);
    let mut rng = Rng::new_with_seed(Algo::MT, CONFIG.bsp.seed);
    let mut frames: Vec<Room> = Vec::new();
    bsp.split_recursive(Some(&mut rng), CONFIG.bsp.recursion_levels, 
                        CONFIG.bsp.min_horizontal_size, 
                        CONFIG.bsp.min_vertical_size, 
                        CONFIG.bsp.max_horizontal_ratio, 
                        CONFIG.bsp.max_vertical_ratio);
    bsp.traverse(TraverseOrder::LevelOrder, | node | {
        if node.is_leaf() {
            frames.push(
                Room {
                    xy: (node.x, node.y),
                    wh: (node.w, node.h),
                    walls: Vec::new(),
                }
            );
        }
        return true;
    });
    return frames;
}

//single_room(&mut ret.0, &mut ret.1, node.x, node.y, node.w, node.h, &rng, CONFIG.bsp.frame, CONFIG.bsp.min_area);
pub fn bsp_gen() -> (Map, Vec<Box<Tile>>) {
    let mut map: (Map, Vec<Box<Tile>>) = empty_gen((CONFIG.game.screen_width, CONFIG.game.screen_height));

    let frames = generate_frames();
    let rooms  = generate_rooms(&frames);

    map = add_to_map(rooms, map.0, map.1);

    if CONFIG.bsp.frame {
        map = add_to_map(frames, map.0, map.1);
    }
    return map;
}
