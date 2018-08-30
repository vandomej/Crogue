use tcod::console::*;

pub trait Tile {
    fn new(x: i32, y: i32) -> Self where Self: Sized;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn get_walkable(&self) -> bool;
    fn get_see_through(&self) -> bool;
    fn draw(&self, window: &Root);
}


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

    fn get_x(&self) -> i32 {
        return self.x;
    }

    fn get_y(&self) -> i32 {
        return self.y;
    }

    fn get_walkable(&self) -> bool {
        return self.walkable;
    }

    fn get_see_through(&self) -> bool {
        return self.see_through;
    }

    fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '#', BackgroundFlag::None);
    }
}


pub struct Floor {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool
}

impl Tile for Floor {
    fn new(x: i32, y: i32) -> Floor {
        return Floor{x, y, walkable: true, see_through: true};
    }

    fn get_x(&self) -> i32 {
        return self.x;
    }

    fn get_y(&self) -> i32 {
        return self.y;
    }

    fn get_walkable(&self) -> bool {
        return self.walkable;
    }

    fn get_see_through(&self) -> bool {
        return self.see_through;
    }

    fn draw(&self, mut window: &Root) {
        window.put_char(self.x, self.y, '.', BackgroundFlag::Set);
    }
}
