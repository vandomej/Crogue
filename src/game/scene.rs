use tcod::input::Key;
use tcod::console::*;

use super::actors::player::Player;
use game::map::mapgen;
use game::map::tile::Tile;
use std::env;


pub struct Scene {
    player: Player,
    map: Vec<Vec<Box<Tile>>>
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            player: Player::new(25, 25),
            map: Scene::gen_map()
        }
    }

    fn gen_map() -> Vec<Vec<Box<Tile>>> {
        let a: Vec<String> = env::args().collect();
        return mapgen::bsp_gen(a[1].parse().unwrap(),
                               a[2].parse().unwrap(),
                               a[3].parse().unwrap(),
                               a[4].parse().unwrap(),
                               a[5].parse().unwrap(),
                               a[6].parse().unwrap());
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
