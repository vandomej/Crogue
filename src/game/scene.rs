use tcod::input::Key;
use tcod::console::*;

use super::actors::player::Player;
use super::map::tile::Wall;
use game::map::tile::Tile;

#[derive(Debug)]
pub struct Scene {
    player: Player,
    wall: Wall
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            player: Player::new(25, 25),
            wall: Wall::new(26, 26)
        }
    }

    pub fn update(&mut self, key: Option<Key>) {
        self.player.update(key);
    }

    pub fn draw(&self, window: &Root) {
        self.player.draw(window);
        self.wall.draw(window);
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);
    }
}
