use tcod::colors;
use tcod::console::*;
use rand::{thread_rng, Rng};
use std::cmp;
use std::io;

use game::actors::game_object;

// Weights for determining how to display the health bar
// More vital body parts should have higher weights
const HEAD_WEIGHT: i32 = 4;
const ARM_WEIGHT: i32 = 1;
const TORSO_WEIGHT: i32 = 4;
const LEG_WEIGHT: i32 = 1;

pub trait Health {
    fn get_head(&self) -> i32;
    fn get_arms(&self) -> Vec<i32>;
    fn get_torso(&self) -> i32;
    fn get_legs(&self) -> Vec<i32>;

    fn set_head(&mut self, value: i32);
    fn set_arms(&mut self, value: Vec<i32>) -> Result<(), io::Error>;
    fn set_torso(&mut self, value: i32);
    fn set_legs(&mut self, value: Vec<i32>) -> Result<(), io::Error>;

    fn is_dead(&self) -> bool;

    fn calculate_damage(&mut self, amount: i32) {
        let number_of_arms: i32 = self.get_arms().len() as i32;
        let number_of_legs: i32 = self.get_legs().len() as i32;

        let mut rng = thread_rng();
        let random: i32 = rng.gen_range(0, 2 + number_of_arms + number_of_legs);

        if random >= 0 && random < number_of_arms { //arms
            let mut arms = self.get_arms();
            let val = arms.remove(random as usize);
            arms.insert(random as usize, cmp::max(val - amount, 0));
            self.set_arms(arms);
        }
        else if random >= number_of_arms && random < number_of_arms + number_of_legs { //legs
            let mut legs = self.get_legs();
            let val = legs.remove((random - number_of_arms) as usize);
            legs.insert((random - number_of_arms) as usize, cmp::max(val - amount, 0));
            self.set_legs(legs);
        }
        else if random == number_of_arms + number_of_legs { //head
            let val = self.get_head();
            self.set_head(cmp::max(val - amount, 0));
        }
        else { //torso
            let val = self.get_torso();
            self.set_torso(cmp::max(val - amount, 0));
        }
    }
}

pub fn draw_health_bar<T>(object: &T, mut window: &Root) 
    where T: Health + game_object::GameObject
{
    let (x, y) = object.get_position();

    //Getting all of the health values
    let mut values: Vec<i32> = Vec::new();
    let mut max_health = 0;

    values.push(object.get_head() * HEAD_WEIGHT);
    max_health += 100 * HEAD_WEIGHT;

    for &i in &object.get_arms() {
        values.push(i * ARM_WEIGHT);
        max_health += 100 * ARM_WEIGHT;
    }
    
    values.push(object.get_torso() * TORSO_WEIGHT);
    max_health += 100 * TORSO_WEIGHT;
    
    for &i in &object.get_legs() {
        values.push(i * LEG_WEIGHT);
        max_health += 100 * LEG_WEIGHT;
    }

    //Calculating the average health from all health values
    let length = values.len() as i32;
    let sum: i32 = values.into_iter().sum();
    let total_health = (sum as f64 / length as f64).round() as i32;
    max_health = (max_health as f64 / length as f64).round() as i32;

    //Setup before displaying health
    let foreground_color = 
        if total_health <= 33 {
            colors::DARK_RED
        }
        else if total_health >= 66 {
            colors::DARK_GREEN
        }
        else {
            colors::DARK_YELLOW
        };
    let background_color = window.get_default_background();

    let line_length = (if object.is_dead() { 0 } else { ((total_health as f64) / (max_health as f64 / 3.0)).ceil() as i32 });

    //Displaying a line on the screen (196 = horizontal line)
    for i in 0..line_length {
        let x_position = x - 1 + i;
        let y_position = y - 1;
        if x_position >= 0 && y_position >= 0 && x_position < window.width() {
            window.put_char_ex(x_position, y_position, 196 as char, foreground_color, background_color)
        }
    }
}