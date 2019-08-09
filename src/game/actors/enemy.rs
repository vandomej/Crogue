use std::io;
use tcod::console::*;
use tcod::map::Map;
use tcod::pathfinding::Dijkstra;

use game::actors::game_object::GameObject;
use game::actors::health::Health;
use game::actors::player::Player;

pub struct Enemy {
    pub x: i32,
    pub y: i32,
    head: i32,
    arms: Vec<i32>,
    torso: i32,
    legs: Vec<i32>,
    attack_cooldown: i32,
    attack_time: i32, // The number of frames in between each attack
    map: Dijkstra<'static>,
    damage: i32,
    symbol: char,
}

impl Enemy {
    pub fn new(x: i32, y: i32, attack_cooldown: i32, map: &Map) -> Enemy {
        return Enemy {
            x,
            y,
            head: 100,
            arms: vec![100, 100],
            torso: 100,
            legs: vec![100, 100],
            attack_cooldown,
            attack_time: 0,
            map: Dijkstra::new_from_map(map.clone(), 0_f32),
            damage: 15,
            symbol: 'E',
        };
    }

    pub fn update(&mut self, player: &mut Player, recalculate: bool) {
        if recalculate == true {
            let (x, y) = player.get_position();
            self.recalculate_dijkstra(x, y);
        }
        
        // When attack_time reaches attack_cooldown, the enemy can attack
        if self.attack_time < self.attack_cooldown {
            self.attack_time += 1;
        } 
        else if player.is_adjacent_to(self) {
            player.calculate_damage(self.damage);
            self.attack_time = 0;
        }
        else if let Some((x, y)) = self.map.walk_one_step() {
            self.set_position(x, y);
            self.attack_time = 0;
        }
    }

    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }

    fn recalculate_dijkstra(&mut self, x: i32, y: i32) {
        let root = self.get_position();
        self.map.compute_grid(root);
        self.map.find((x, y));
    }
}

implement_health!(Enemy);

implement_gameobject!(Enemy);