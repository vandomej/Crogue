use tcod::console::*;

#[derive(Debug)]
pub struct Wall{
    pub x: i32,
    pub y: i32
}

impl Wall{
    pub fn new(x: i32, y: i32) -> Wall{
        return Wall{x, y};
    }
    pub fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '#', BackgroundFlag::None);
    }
}

pub trait BlockProperties{
    fn walkeable(&self)->bool{
        return true;
    }
    fn see_through(&self)->bool{
        return true;
    }
}



impl BlockProperties for Wall{
    fn walkeable(&self)->bool{
        return false;
    }
    fn see_through(&self)->bool{
        return false;
    }
} 