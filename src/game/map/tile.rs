use tcod::console::*;


pub trait Tile {
    fn new(x: i32, y: i32) -> Self where Self: Sized;

    fn draw(&self, window: &Root);
}


#[derive(Debug)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool
}

impl Tile for Wall {
    fn new(x: i32, y: i32) -> Wall {
        return Wall{x, y, walkable: false, see_through: false};
    }

    fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '#', BackgroundFlag::None);
    }
}
