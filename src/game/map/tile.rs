use tcod::console::*;



pub trait Tile{
    fn new(x: i32, y: i32) -> Self;

    fn walkeable(&self)->bool{
        return true;
    }
    fn see_through(&self)->bool{
        return true;
    }

    fn draw(&self, window: &Root);
}


#[derive(Debug)]
pub struct Wall{
    pub x: i32,
    pub y: i32
}

impl Tile for Wall{
    fn new(x: i32, y: i32) -> Wall{
        Wall{x: x, y: y}
    }

    fn walkeable(&self)->bool{
        return false;
    }

    fn see_through(&self)->bool{
        return false;
    }
    
    fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '#', BackgroundFlag::None);
    }
} 