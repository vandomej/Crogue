use tcod::input::Key;
use tcod::console::*;
use tcod::colors;
use std::io;

use game::map::tile::Tile;
use game::actors::health::Health;
use game::actors::game_object::GameObject;

#[derive(Debug, Clone)]
pub struct Player {
    pub xy: (i32, i32),
    head: i32,
    arms: Vec<i32>,
    torso: i32,
    legs: Vec<i32>,
    symbol: char
}

impl Player {
    pub fn new(xy: (i32, i32)) -> Player {
        return Player {
            xy,
            head: 100,
            arms: vec![100, 100],
            torso: 100,
            legs: vec![100, 100],
            symbol: '@',
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
        GameObject::move_object(self, tiles, proposed_position).unwrap()
    }

    pub fn draw_hud(&self, window: &Root) {
        self.draw_health(self.head, "H ", 0, window);
        self.draw_health(self.arms[0], "AL", 1, window);
        self.draw_health(self.arms[1], "AR", 2, window);
        self.draw_health(self.torso, "T ", 3, window);
        self.draw_health(self.legs[0], "LL", 4, window);
        self.draw_health(self.legs[1], "LR", 5, window);
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

implement_health!(Player);

implement_gameobject!(Player);