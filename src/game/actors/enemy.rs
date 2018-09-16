use std::io;
use tcod::console::*;

use game::map::tile::Tile;
use game::actors::player::Player;
use game::actors::health;
use game::actors::game_object;

#[derive(Debug)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    head: i32,
    arms: (i32, i32),
    torso: i32,
    legs: (i32, i32),
    player: Player
}

impl Enemy {
    pub fn new(x: i32, y: i32, player: Player) -> Enemy {
        return Enemy {
            x, 
            y,
            head: 100,
            arms: (100, 100),
            torso: 100,
            legs: (100, 100),
            player
        };
    }

    pub fn update(&mut self, tiles: &Vec<Box<Tile>>, player: &Player) {
        self.player = player.clone(); // Might be better to store a reference to the player rather than replacing player data on update but lifetimes make it too much trouble
    }

    pub fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, 'E', BackgroundFlag::Set);
    }
    
    pub fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }
}

impl health::Health for Enemy {
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

    fn set_arms(&mut self, value: Vec<i32>) -> Result<(), io::Error>{
        if value.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error setting arm health for enemy. This enemy has two arms."));
        }

        self.arms.0 = value[0];
        self.arms.1 = value[1];

        Ok(())
    }

    fn set_torso(&mut self, value: i32) {
        self.torso = value;
    }

    fn set_legs(&mut self, value: Vec<i32>) -> Result<(), io::Error> {
        if value.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error setting leg health for enemy. This enemy has two legs."));
        }

        self.legs.0 = value[0];
        self.legs.1 = value[1];

        Ok(())
    }

    fn is_dead(&self) -> bool {
        return !(self.head > 0 && self.torso > 0);
    }
}

impl game_object::GameObject for Enemy {
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn move_object(&self, position: (i32, i32)) {

    }
}