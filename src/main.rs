extern crate tcod;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod game;
pub mod config;

use tcod::console::*;
use tcod::colors;
use tcod::input::KeyPressFlags;

use self::game::Game;
use self::config::*;


fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(CONFIG.game.screen_width, CONFIG.game.screen_height)
        .title("crogue")
        .init();

    tcod::system::set_fps(CONFIG.game.fps_limit);

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
