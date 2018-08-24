mod player;

use std::{thread, time};
use player::Player;


fn main() {
    let hundred_millis = time::Duration::from_millis(100);
    let mut player = Player::new(250, 250);

    while(true) {
        let x = player.x + 1;
        let y = player.y + 1;
        player.set_pos(x, y);
        println!("{:#?}", player);

        thread::sleep(hundred_millis);  // rust is so fast I have to do this to get useful output
    }
}
