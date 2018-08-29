use tcod::input::Key;
use tcod::console::*;

use super::actors::player::Player;
use game::map::mapgen;
use game::map::tile::Tile;


pub struct Scene {
    player: Player,
    map: Vec<Vec<Box<Tile>>>
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            player: Player::new(25, 25),
            map: mapgen::dummy_gen(45, 45)
        }
    }

    pub fn update(&mut self, key: Option<Key>) {
        self.player.update(key, &self.map);
    }

    pub fn draw(&self, window: &Root) {
        for container in &self.map {
            for elem in container {
                elem.draw(window);
            }
        }

        self.player.draw(window);
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);
    }
}
