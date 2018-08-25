use tcod::input::Key;
use tcod::console::*;

use super::actors::player::Player;


#[derive(Debug)]
pub struct Scene {
    player: Player
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            player: Player:: new(25, 25)
        }
    }

    pub fn update(&mut self, key: Option<Key>) {
        self.player.update(key);
    }

    pub fn draw(&self, window: &Root) {
        self.player.draw(window);
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);
    }
}
