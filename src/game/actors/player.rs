use tcod::input::Key;
use tcod::console::*;
use tcod::colors;
use std::io;

use game::map::tile::Tile;
use game::actors::health;
use game::actors::game_object;

#[derive(Debug, Clone)]
pub struct Player {
    pub xy: (i32, i32),
    head: i32,
    arms: (i32, i32),
    torso: i32,
    legs: (i32, i32)
}

impl Player {
    pub fn new(xy: (i32, i32)) -> Player {
        return Player {
            xy,
            head: 100,
            arms: (100, 100),
            torso: 100,
            legs: (100, 100)
        };
    }

    pub fn update(&mut self, key: Option<Key>, tiles: &Vec<Box<Tile>>) -> bool {
        return if key.is_some() && key.unwrap().pressed {
            self.handle_key(key, tiles)
        } else {
            false
        }
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

        let (x, y) = self.xy;
        let proposed_position = (x + proposed_x, y + proposed_y);
        game_object::GameObject::move_object(self, tiles, proposed_position).unwrap()
    }

    pub fn draw_hud(&self, window: &Root) {
        self.draw_health(self.head, "H ", 0, window);
        self.draw_health(self.arms.0, "AL", 1, window);
        self.draw_health(self.arms.1, "AR", 2, window);
        self.draw_health(self.torso, "T ", 3, window);
        self.draw_health(self.legs.0, "LL", 4, window);
        self.draw_health(self.legs.1, "LR", 5, window);
    }

    fn draw_health(&self, health: i32, label: &str, row: i32, mut window: &Root) {
        let line = format!("{:2} {: >4} ", label, health);
        let foreground_color = 
            if health <= 33 {
                colors::DARK_RED
            }
            else if health >= 66 {
                colors::DARK_GREEN
            }
            else {
                colors::WHITE
            };
        let background_color = window.get_default_background();

        for (i, c) in line.chars().enumerate() {
            window.put_char_ex(i as i32, row, c, foreground_color, background_color)
        }
    }

    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.xy.0, self.xy.1, ' ', BackgroundFlag::Set);
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

    fn set_arms(&mut self, value: Vec<i32>) -> Result<Vec<i32>, io::Error>{
        if value.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error setting arm health for player, players require at least 2 arm health values to be provided."));
        }

        self.arms.0 = value[0];
        self.arms.1 = value[1];

        Ok(value)
    }

    fn set_torso(&mut self, value: i32) {
        self.torso = value;
    }

    fn set_legs(&mut self, value: Vec<i32>) -> Result<Vec<i32>, io::Error> {
        if value.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error setting leg health for player, players require at least 2 leg health values to be provided."));
        }

        self.legs.0 = value[0];
        self.legs.1 = value[1];

        Ok(value)
    }

    fn is_dead(&self) -> bool {
        return !(self.head > 0 && self.torso > 0);
    }
}

impl game_object::GameObject for Player {
    fn get_position(&self) -> (i32, i32) {
        self.xy
    }

    fn set_position(&mut self, xy: (i32, i32)) {
        self.xy = xy;
    }

    fn get_symbol(&self) -> char { '@' }
}