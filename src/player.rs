#[derive(Debug)]
pub struct Player {
    pub x: u32,
    pub y: u32
}

impl Player {
    pub fn new(x: u32, y: u32) -> Player {
        Player {x, y}
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}
