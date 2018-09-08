use std::io;

pub trait Health {
    fn get_head(&self) -> i32;
    fn get_arms(&self) -> Vec<i32>;
    fn get_torso(&self) -> i32;
    fn get_legs(&self) -> Vec<i32>;

    fn set_head(&mut self, value: i32);
    fn set_arms(&mut self, value: Vec<i32>) -> Result<(), io::Error>;
    fn set_torso(&mut self, value: i32);
    fn set_legs(&mut self, value: Vec<i32>) -> Result<(), io::Error>;

    fn calculate_damage(&mut self, amount: i32);
}