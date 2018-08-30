use tcod::input::Key;
use tcod::console::*;
use tcod::map::Map;
use tcod::map::FovAlgorithm;

use super::actors::player::Player;
use game::map::mapgen;
use game::map::tile::Tile;


pub struct Scene {
    player: Player,
    map: Map,
    tiles: Vec<Box<Tile>>,
    recalc_fov: bool
}

impl Scene {
    pub fn new() -> Scene {
        let (map, tiles) = mapgen::dummy_gen(45, 45);
        return Scene {
            player: Player::new(26, 25),
            recalc_fov: true,
            map,
            tiles,
        }
    }

    pub fn update(&mut self, key: Option<Key>) {
        if self.recalc_fov {
            self.map.compute_fov(self.player.x, self.player.y, 10, true, FovAlgorithm::Basic);
        }

        self.recalc_fov = self.player.update(key, &self.tiles);
    }

    pub fn draw(&self, window: &Root) {
        for tile in &self.tiles {
            if self.map.is_in_fov(tile.get_x(), tile.get_y()) {
                tile.draw(window);
            }
        }

        self.player.draw(window);
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);

        for tile in &self.tiles {
            tile.clear(window);
        }
    }
}
