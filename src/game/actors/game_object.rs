use std::io;

use game::map::tile::Tile;

pub trait GameObject {
    fn get_position(&self) -> (i32, i32);

    fn set_position(&mut self, position: (i32, i32));

    fn move_object(&mut self, tiles: &Vec<Box<Tile>>, position: (i32, i32)) -> Result<bool, io::Error> {
        let (x, y) = self.get_position();
        let (proposed_x, proposed_y) = position;
        if (x - proposed_x).abs() > 1 || (y - proposed_y).abs() > 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cannot move an object further than an adjacent tile, try using set_position instead."));
        }

        for tile in tiles {
            if tile.get_x() == proposed_x &&
            tile.get_y() == proposed_y &&
            tile.get_walkable() == true {
                self.set_position(position);
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn is_adjacent_to<T>(&self, other: &T) -> bool
        where T: GameObject
    {
        let (x1, y1) = self.get_position();
        let (x2, y2) = other.get_position();

        return (x2 <= x1 + 1) && (x2 >= x1 - 1) && (y2 <= y1 + 1) && (y2 >= y1 - 1) 
    }
}