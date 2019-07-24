use tcod::console::*;

pub trait Tile {
    fn new(xy: (i32, i32)) -> Self where Self: Sized;
    fn get_xy(&self) -> (i32, i32);
    fn get_walkable(&self) -> bool;
    fn get_see_through(&self) -> bool;
    fn draw(&self, window: &Root);
    fn clear(&self, window: &Root);
    fn get_symbol(&self) -> char;
}

macro_rules! implement_tile {
    ($type:ty) => {
        impl Tile for $type {
            fn new(xy: (i32, i32)) -> $type {
                let mut t = <$type>::default();
                t.xy = xy;
                return t;
            }

            fn get_xy(&self) -> (i32, i32) {
                return self.xy;
            }

            fn get_walkable(&self) -> bool {
                return self.walkable;
            }

            fn get_see_through(&self) -> bool {
                return self.see_through;
            }

            fn draw(&self, mut window: &Root) {
                window.put_char(self.xy.0, self.xy.1, self.get_symbol(), BackgroundFlag::None);
            }

            fn clear(&self, mut window: &Root) {
                window.put_char(self.xy.0, self.xy.1, ' ', BackgroundFlag::Set);
            }

            fn get_symbol(&self) -> char {
                return self.symbol;
            }
        }
    };
}

pub struct Wall {
    pub xy: (i32, i32),
    pub walkable: bool,
    pub see_through: bool,
    symbol: char,
}

impl Default for Wall {
    fn default() -> Wall {
        Wall {
            xy: (0, 0),
            walkable: false,
            see_through: false,
            symbol: '#',
        }
    }
}

implement_tile!(Wall);

pub struct Floor {
    pub xy: (i32, i32),
    pub walkable: bool,
    pub see_through: bool,
    symbol: char,
}

impl Default for Floor {
    fn default() -> Floor {
        Floor {
            xy: (0, 0),
            walkable: true,
            see_through: true,
            symbol: '.'
        }
    }
}

implement_tile!(Floor);
