use tcod::input::Key;
use tcod::console::*;
use tcod::map::Map;
use tcod::map::FovAlgorithm;

use super::actors::player::Player;
use super::actors::enemy::Enemy;
use game::map::mapgen;
use game::map::tile::Tile;
use config::*;
use game::actors::health;


pub struct Scene {
    player: Player,
    enemies: Vec<Enemy>,
    map: Map,
    tiles: Vec<Box<Tile>>,
    recalc_fov: bool
}

impl Scene {
    pub fn new() -> Scene {
        let (map, tiles) = mapgen::bsp_gen();
        return Scene {
            player: Player::new(27, 25),
            enemies: vec![Enemy::new(32, 25, 10)],
            recalc_fov: true,
            map,
            tiles,
        }
    }

    pub fn update(&mut self, key: Option<Key>) {
        let fov = if CONFIG.game.see_all { 300 } else { CONFIG.game.fov };
        if self.recalc_fov {
            self.map.compute_fov(self.player.x, self.player.y, fov, true, FovAlgorithm::Basic);
        }

        self.recalc_fov = self.player.update(key, &self.tiles);

        for enemy in &mut self.enemies {
            enemy.update(&self.tiles, &mut self.player);
        }
    }

    pub fn draw(&self, window: &Root) {
        for tile in &self.tiles {
            if self.map.is_in_fov(tile.get_x(), tile.get_y()) {
                tile.draw(window);
            }
        }

        for enemy in &self.enemies {
            health::draw_health_bar(enemy, window);
            enemy.draw(window);
        }
        health::draw_health_bar(&self.player, window);
        
        self.player.draw(window);
        self.player.draw_hud(window);
        
        
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);

        for tile in &self.tiles {
            tile.clear(window);
        }
    }
}
