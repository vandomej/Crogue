use tcod::input::Key;
use tcod::console::*;
use tcod::map::Map;
use tcod::map::FovAlgorithm;

use super::actors::player::Player;
use game::map::mapgen;
use game::map::tile::Tile;
use std::env;


pub struct Scene {
    player: Player,
    map: Map,
    tiles: Vec<Box<Tile>>,
    recalc_fov: bool
}

impl Scene {
    pub fn new() -> Scene {
        let (map, tiles) = Scene::gen_map();
        return Scene {
            player: Player::new(26, 25),
            recalc_fov: true,
            map,
            tiles,
        }
    }

    fn gen_map() -> (Map, Vec<Box<Tile>>) {
        let a: Vec<String> = env::args().collect();
        return mapgen::bsp_gen(a[1].parse().unwrap(),
                               a[2].parse().unwrap(),
                               a[3].parse().unwrap(),
                               a[4].parse().unwrap(),
                               a[5].parse().unwrap(),
                               a[6].parse().unwrap(),
                               a[7].parse().unwrap(),
                               a[8].parse().unwrap());
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
