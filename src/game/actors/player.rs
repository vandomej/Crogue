use tcod::input::Key;
use tcod::console::*;
use tcod::colors;
use std::io;
use rand::{thread_rng, Rng};

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
            Some(Key { code: Char, printable: 'd', ..}) => health::Health::calculate_damage(self, 15),
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
        self.draw_hud(window);
    }

    fn draw_hud(&self, window: &Root) {
        let window = self.draw_health(self.head, 0, window);
        let window = self.draw_health(self.arms.0, 1, window);
        let window = self.draw_health(self.arms.1, 2, window);
        let window = self.draw_health(self.torso, 3, window);
        let window = self.draw_health(self.legs.0, 4, window);
        self.draw_health(self.legs.1, 5, window);
    }

    fn draw_health<'a>(&self, health: i32, row: i32, mut window: &'a Root) -> &'a Root {
        let line = format!("{: >4} ", health);
        let foreground_color = 
            if health <= 25 {
                colors::DARK_RED
            }
            else if health >= 75 {
                colors::DARK_GREEN
            }
            else {
                colors::WHITE
            };
        let background_color = window.get_default_background();

        for (i, c) in line.chars().enumerate() {
            window.put_char_ex(i as i32, row, c, foreground_color, background_color)
        }
        window
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
        self.torso = value;
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
        let mut rng = thread_rng();
        let random: i32 = rng.gen_range(0, 6);

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
