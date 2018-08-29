use tcod::input::Key;
use tcod::console::*;

use super::actors::player::Player;
use game::map::mapgen;
use game::map::tile::Tile;


pub struct Scene {
    player: Player,
    map: Vec<Box<Tile>>
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            player: Player::new(25, 25),
            map: mapgen::bsp_gen(5, 5, 5, 0.4, 0.5)
        }
    }

    pub fn update(&mut self, key: Option<Key>) {
        self.player.update(key, &self.map);
    }

    pub fn draw(&self, window: &Root) {
        self.player.draw(window);

        for elem in &self.map {
            elem.draw(window);
        }
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);
    }
}
