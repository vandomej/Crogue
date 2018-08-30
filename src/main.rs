extern crate tcod;

mod game;

use tcod::console::*;
use tcod::colors;
use tcod::input::KeyPressFlags;

use self::game::Game;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS_LIMIT: i32 = 30;

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("crogue")
        .init();

    tcod::system::set_fps(FPS_LIMIT);

    let mut game = Game::new();

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);

        let key = root.check_for_keypress(KeyPressFlags::all());

        let game_data = game.update(key);
        game.draw(&mut root);

        root.flush();

        game.clear(&mut root);

        if game_data.quit == true {
            break
        }
    }
}
