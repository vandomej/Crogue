use tcod::input::Key;
use tcod::console::*;
use std::io;

use game::map::tile::Tile;
use game::actors::health;


#[derive(Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    head: i32,
    arms: (i32, i32),
    torso: i32,
    legs: (i32, i32)
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        return Player {
            x, 
            y,
            head: 100,
            arms: (100, 100),
            torso: 100,
            legs: (100, 100)
        };
    }

    pub fn update(&mut self, key: Option<Key>, tiles: &Vec<Box<Tile>>) -> bool {
        let mut recalc_fov = false;

        if key.is_some() && key.unwrap().pressed {
            recalc_fov = self.handle_key(key, tiles);
        }

        return recalc_fov;
    }

    fn handle_key(&mut self, key: Option<Key>, tiles: &Vec<Box<Tile>>) -> bool {
        use tcod::input::KeyCode::*;

        let mut proposed_x = 0;
        let mut proposed_y = 0;

        match key {
            Some(Key { code: Up, .. }) => proposed_y -= 1,
            Some(Key { code: Down, .. }) => proposed_y += 1,
            Some(Key { code: Left, .. }) => proposed_x -= 1,
            Some(Key { code: Right, .. }) => proposed_x += 1,
            _ => {},
        }

        for tile in tiles {
            if tile.get_x() == (self.x + proposed_x) &&
               tile.get_y() == (self.y + proposed_y) &&
               tile.get_walkable() == false {
               return false;
            }
        }

        self.x += proposed_x;
        self.y += proposed_y;
        return true;
    }

    pub fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '@', BackgroundFlag::Set);
    }

    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }
}

impl health::Health for Player {
    fn get_head(&self) -> i32 {
        self.head
    }

    fn get_arms(&self) -> Vec<i32> {
        vec![self.arms.0, self.arms.1]
    }

    fn get_torso(&self) -> i32 {
        self.torso
    }

    fn get_legs(&self) -> Vec<i32> {
        vec![self.legs.0, self.legs.1]
    }

    fn set_head(&mut self, value: i32) {
        self.head = value;
    }

    fn set_arms(&mut self, value: Vec<i32>) -> Result<(), io::Error>{
        if value.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error setting arm health for player, players require at least 2 arm health values to be provided."));
        }

        self.arms.0 = value[0];
        self.arms.1 = value[1];

        Ok(())
    }

    fn set_torso(&mut self, value: i32) {
        self.torso = value
    }

    fn set_legs(&mut self, value: Vec<i32>) -> Result<(), io::Error> {
        if value.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error setting leg health for player, players require at least 2 leg health values to be provided."));
        }

        self.legs.0 = value[0];
        self.arms.1 = value[1];

        Ok(())
    }

    fn calculate_damage(&mut self, amount: i32) {
        //get random number between 0 and 5
        //subtract amount from the body part provided by the random number
        let random = 3;

        match random {
            0 => {self.head -= amount},
            1 => {self.arms.0 -= amount},
            2 => {self.arms.1 -= amount},
            3 => {self.torso -= amount},
            4 => {self.legs.0 -= amount},
            5 => {self.legs.1 -= amount},
            _ => ()
        }
    }
}
