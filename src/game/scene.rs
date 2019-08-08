use tcod::input::Key;
use tcod::console::*;
use tcod::map::Map;
use tcod::map::FovAlgorithm;
use tcod::colors;

use super::actors::player::Player;
use super::actors::enemy::Enemy;
use game::map::mapgen;
use game::map::tile::Tile;
use game::map::tile::SceneTransitionType;
use config::*;
use game::actors::health;
use game::actors::game_object::GameObject;
use game::actors::health::Health;


pub struct Scene {
    player: Player,
    enemies: Vec<Enemy>,
    map: Map,
    tiles: Vec<Box<Tile>>,
    recalculate_map: bool
}

impl Scene {
    pub fn new(seed: u32, first_floor: bool) -> Scene {
        let (map, tiles, player_spawn) = mapgen::bsp_gen(seed, first_floor);
        return Scene {
            player: Player::new(player_spawn.0, player_spawn.1),
            enemies: vec![
                Enemy::new(70, 25, 10, &map),
                Enemy::new(50, 30, 10, &map)],
            recalculate_map: true,
            map,
            tiles,
        };
    }

    pub fn update(&mut self, key: Option<Key>) -> Option<SceneTransitionType> {
        let mut scene_transition_type = None;
        let fov = if CONFIG.game.see_all { 300 } else { CONFIG.game.fov };
        if self.recalculate_map {
            self.map.compute_fov(self.player.x, self.player.y, fov, true, FovAlgorithm::Basic);
        }

        for enemy in &mut self.enemies {
            enemy.update(&mut self.player, self.recalculate_map);
        }

        self.recalculate_map = self.player.update(key, &self.tiles);

        for tile in &self.tiles {
            if self.player.get_position() == (tile.get_x(), tile.get_y()) && self.recalculate_map {
                match tile.causes_scene_transitions() {
                    Some(SceneTransitionType::Down) => { scene_transition_type = Some(SceneTransitionType::Down) },
                    Some(SceneTransitionType::Up) => { scene_transition_type = Some(SceneTransitionType::Up) },
                    None => {}
                }
            }
        }

        return scene_transition_type;
    }

    pub fn draw(&self, window: &Root) {
        if self.player.is_dead() {
            self.draw_player_death_screen(&window);
        } else {
            self.draw_scene(&window);
        }
    }

    fn draw_scene(&self, window: &Root) {
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

    fn draw_player_death_screen(&self, mut window: &Root) {
        let foreground_color = colors::WHITE;
        let background_color = window.get_default_background();

        let mut line = format!("{:2}", "YOU HAVE DIED.");

        for (i, c) in line.chars().enumerate() {
            window.put_char_ex(i as i32, 1, c, foreground_color, background_color)
        }
    }

    pub fn clear(&self, window: &Root) {
        self.player.clear(window);

        for tile in &self.tiles {
            tile.clear(window);
        }
    }
}
