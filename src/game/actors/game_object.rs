use std::io;
use tcod::console::*;

use game::map::tile::Tile;

pub trait GameObject {
    fn get_position(&self) -> (i32, i32);

    fn set_position(&mut self, xy: (i32, i32));

    fn get_symbol(&self) -> char;

    fn draw(&self, mut window: &Root) {
        let (x, y) = self.get_position();
        window.put_char(x, y, self.get_symbol(), BackgroundFlag::Set);
    }

    fn move_object(&mut self, tiles: &Vec<Box<Tile>>, position: (i32, i32)) -> Result<bool, io::Error> {
        let (x, y) = self.get_position();
        let (proposed_x, proposed_y) = position;
        if (x - proposed_x).abs() > 1 || (y - proposed_y).abs() > 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cannot move an object further than an adjacent tile, try using set_position instead."));
        }

        for tile in tiles {
            if tile.get_xy() == position &&
            tile.get_walkable() == false {
                return Ok(false);
            }
        }

        self.set_position(position);
        Ok(true)
    }

    fn is_adjacent_to(&self, other: &GameObject) -> bool
    {
        let (x1, y1) = self.get_position();
        let (x2, y2) = other.get_position();

        return (x2 <= x1 + 1) && (x2 >= x1 - 1) && (y2 <= y1 + 1) && (y2 >= y1 - 1) 
    }
}


macro_rules! implement_gameobject {
    ($type:ty) => {
        impl GameObject for $type {
            fn get_position(&self) -> (i32, i32) {
                return self.xy;
            }

            fn set_position(&mut self, xy: (i32, i32)) {
                self.xy = xy;
            }

            fn get_symbol(&self) -> char {
                self.symbol
            }
        }
        
    };
}
