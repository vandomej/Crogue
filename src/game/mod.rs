mod scene;
mod actors;
mod map;

use tcod::input::Key;
use tcod::console::*;

use self::scene::Scene;


pub struct Game {
    scene: Scene
}

#[derive(Debug)]
pub struct GameData {
    pub quit: bool
}

impl Game {
    pub fn new() -> Game {
        return Game {
            scene: Scene::new()
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

        self.scene.update(key);

        return game_data;
    }

    pub fn draw(&self, window: &Root) {
        self.scene.draw(window);
    }

    pub fn clear(&self, window: &Root) {
        self.scene.clear(window);
    }
}
