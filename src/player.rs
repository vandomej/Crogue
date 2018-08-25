#[derive(Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        return Player {x, y}
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
