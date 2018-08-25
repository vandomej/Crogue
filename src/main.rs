extern crate tcod;

mod player;

use std::{thread, time};
use tcod::console::*;
use tcod::colors;
use tcod::input::KeyPressFlags;
use player::Player;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;


fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.check_for_keypress(KeyPressFlags::empty());
    match key {
        Some(Key { code: Enter, alt: true, .. }) => {

            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Some(Key { code: Escape, .. }) => return true,  // exit game

        // movement keys
        Some(Key { code: Up, .. }) => *player_y -= 1,
        Some(Key { code: Down, .. }) => *player_y += 1,
        Some(Key { code: Left, .. }) => *player_x -= 1,
        Some(Key { code: Right, .. }) => *player_x += 1,

        _ => {},
    }

    return false;
}


fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("crogue")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Player::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(player.x, player.y, '@', BackgroundFlag::None);

        root.flush();

        root.put_char(player.x, player.y, ' ', BackgroundFlag::None);

        // handle keys and exit game if needed
        let exit = handle_keys(&mut root, &mut player.x, &mut player.y);

        if exit {
            break
        }
    }
}
