mod scene;
mod actors;
mod map;

use tcod::input::Key;
use tcod::console::*;

use config::*;

use self::scene::Scene;
use game::map::tile::SceneTransitionType;


pub struct Game {
    scenes: Vec<Scene>,
    curr_scene_idx: usize,
    used_seeds: Vec<u32>
}

#[derive(Debug)]
pub struct GameData {
    pub quit: bool
}

impl Game {
    pub fn new() -> Game {
        return Game {
            scenes: vec![Scene::new(CONFIG.bsp.seed, true)],
            curr_scene_idx: 0,
            used_seeds: vec![CONFIG.bsp.seed]
        };
    }

    pub fn update(&mut self, key: Option<Key>) -> GameData {
        use tcod::input::KeyCode::*;

        let mut game_data = GameData {
            quit: false
        };

        match key {
            Some(Key { code: Escape, .. }) => {
                game_data.quit = true
            },
            _ => {},
        }

        match self.scenes[self.curr_scene_idx].update(key) {
            Some(SceneTransitionType::Up) => self.scene_transition(SceneTransitionType::Up),
            Some(SceneTransitionType::Down) => self.scene_transition(SceneTransitionType::Down),
            None => {}
        }

        return game_data;
    }

    pub fn draw(&self, window: &Root) {
        self.scenes[self.curr_scene_idx].draw(window);
    }

    pub fn clear(&self, window: &Root) {
        self.scenes[self.curr_scene_idx].clear(window);
    }

    fn scene_transition(&mut self, scene_transition_type: SceneTransitionType) {
        match scene_transition_type {
            SceneTransitionType::Up => self.scene_transition_up(),
            SceneTransitionType::Down => self.scene_transition_down()
        }
    }

    fn scene_transition_up(&mut self) {
        if self.curr_scene_idx == 0 {
            let seed = self.generate_unique_seed();
            self.scenes.insert(0, Scene::new(seed, false));
            self.used_seeds.push(seed);
            self.curr_scene_idx = 0;
        } else {
            self.curr_scene_idx -= 1;
        }
    }

    fn scene_transition_down(&mut self) {
        if self.curr_scene_idx == self.scenes.len()-1 {
            let seed = self.generate_unique_seed();
            self.scenes.push(Scene::new(seed, false));
            self.used_seeds.push(seed);
        }

        self.curr_scene_idx += 1;
    }

    fn generate_unique_seed(&self) -> u32 {
        let seed = self.used_seeds[self.used_seeds.len()-1] + 1;
        seed
    }
}
