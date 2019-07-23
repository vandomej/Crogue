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
    arms: (i32, i32),
    torso: i32,
    legs: (i32, i32),
    attack_cooldown: i32,
    attack_time: i32, // The number of frames in between each attack
    map: Dijkstra<'static>,
}

impl Enemy {
    pub fn new(x: i32, y: i32, attack_cooldown: i32, map: &Map) -> Enemy {
        return Enemy {
            x,
            y,
            head: 100,
            arms: (100, 100),
            torso: 100,
            legs: (100, 100),
            attack_cooldown,
            attack_time: 0,
            map: Dijkstra::new_from_map(map.clone(), 0_f32),
        };
    }

    pub fn update(&mut self, player: &mut Player, recalculate: bool) {
        if recalculate == true {
            self.recalculate_dijkstra(player.get_position());
        }
        
        // When attack_time reaches attack_cooldown, the enemy can attack
        if self.attack_time < self.attack_cooldown {
            self.attack_time += 1;
        } 
        else if player.is_adjacent_to(self) {
            player.calculate_damage(15);
            self.attack_time = 0;
        }
        else if let Some(position) = self.map.walk_one_step() {
            self.set_position(position);
            self.attack_time = 0;
        }
    }

    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }

    fn recalculate_dijkstra(&mut self, destination: (i32, i32)) {
        let root = self.get_position();
        self.map.compute_grid(root);
        self.map.find(destination);
    }
}

impl Health for Enemy {
    fn get_head(&self) -> i32 {
        self.head
    }

    fn get_arms(&self) -> Vec<i32> {
        vec![self.arms.0, self.arms.1]
    }

    fn get_torso(&self) -> i32 {
        self.torso
    }

    fn get_legs(&self) -> Vec<i32> {
        vec![self.legs.0, self.legs.1]
    }

    fn set_head(&mut self, value: i32) {
        self.head = value;
    }

    fn set_arms(&mut self, value: Vec<i32>) -> Result<Vec<i32>, io::Error> {
        if value.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error setting arm health for enemy. This enemy has two arms.",
            ));
        }

        self.arms.0 = value[0];
        self.arms.1 = value[1];

        Ok(value)
    }

    fn set_torso(&mut self, value: i32) {
        self.torso = value;
    }

    fn set_legs(&mut self, value: Vec<i32>) -> Result<Vec<i32>, io::Error> {
        if value.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error setting leg health for enemy. This enemy has two legs.",
            ));
        }

        self.legs.0 = value[0];
        self.legs.1 = value[1];

        Ok(value)
    }

    fn is_dead(&self) -> bool {
        return !(self.head > 0 && self.torso > 0);
    }
}

impl GameObject for Enemy {
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.x = position.0;
        self.y = position.1;
    }

    fn get_symbol(&self) -> char { 'E' }
}