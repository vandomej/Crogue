use tcod::input::Key;
use tcod::console::*;


#[derive(Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        return Player {x, y};
    }

    pub fn update(&mut self, key: Option<Key>) {
        use tcod::input::KeyCode::*;

        match key {
            Some(Key { code: Up, .. }) => self.y -= 1,
            Some(Key { code: Down, .. }) => self.y += 1,
            Some(Key { code: Left, .. }) => self.x -= 1,
            Some(Key { code: Right, .. }) => self.x += 1,
            _ => {},
        }
    }

    pub fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '@', BackgroundFlag::None);
    }

    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}
