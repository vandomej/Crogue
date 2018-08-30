use tcod::input::Key;
use tcod::console::*;

use game::map::tile::Tile;


#[derive(Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        return Player {x, y};
    }

    pub fn update(&mut self, key: Option<Key>, map: &Vec<Vec<Box<Tile>>>) {
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

        if map[(self.y + proposed_y) as usize][(self.x + proposed_x) as usize]
            .get_walkable() == false {

            proposed_x = 0;
            proposed_y = 0;
        }

        self.x += proposed_x;
        self.y += proposed_y;
    }

    pub fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '@', BackgroundFlag::None);
    }

    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}
