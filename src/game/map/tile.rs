use tcod::console::*;

pub enum SceneTransitionType {
    Up,
    Down
}

pub trait Tile {
    fn new(x: i32, y: i32) -> Self where Self: Sized;
    fn get_position(&self) -> (i32, i32);
    fn get_walkable(&self) -> bool;
    fn get_see_through(&self) -> bool;
    fn draw(&self, window: &Root);
    fn clear(&self, window: &Root);
    fn get_symbol(&self) -> char;
    fn causes_scene_transitions(&self) -> &Option<SceneTransitionType>;
}

macro_rules! implement_tile {
    ($type:ty) => {
        impl Tile for $type {
            fn new(x: i32, y: i32) -> $type {
                let mut t = <$type>::default();
                t.x = x;
                t.y = y;
                return t;
            }

            fn get_position(&self) -> (i32, i32) {
                return (self.x, self.y);
            }

            fn get_walkable(&self) -> bool {
                return self.walkable;
            }

            fn get_see_through(&self) -> bool {
                return self.see_through;
            }

            fn draw(&self, mut window: &Root) {
                window.put_char(self.x, self.y, self.get_symbol(), BackgroundFlag::None);
            }

            fn clear(&self, mut window: &Root) {
                window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
            }

            fn get_symbol(&self) -> char {
                return self.symbol;
            }
            
            fn causes_scene_transitions(&self) -> &Option<SceneTransitionType> {
                return &self.causes_scene_transitions;
            }
        }
    };
}

pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub symbol: char,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Default for Wall {
    fn default() -> Wall {
        Wall {
            x: 0,
            y: 0,
            walkable: false,
            see_through: false,
            symbol: '#',
            causes_scene_transitions: None
        }
    }
}

implement_tile!(Wall);

pub struct Floor {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub symbol: char,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Default for Floor {
    fn default() -> Floor {
        Floor {
            x: 0,
            y: 0,
            walkable: true,
            see_through: true,
            symbol: '.',
            causes_scene_transitions: None
        }
    }
}

implement_tile!(Floor);

pub struct StairUp {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub symbol: char,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Default for StairUp {
    fn default() -> StairUp {
        StairUp {
            x: 0,
            y: 0,
            walkable: true,
            see_through: true,
            symbol: '<',
            causes_scene_transitions: Some(SceneTransitionType::Up)
        }
    }
}

implement_tile!(StairUp);

pub struct StairDown {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub symbol: char,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Default for StairDown {
    fn default() -> StairDown {
        StairDown {
            x: 0,
            y: 0,
            walkable: true,
            see_through: true,
            symbol: '>',
            causes_scene_transitions: Some(SceneTransitionType::Down)
        }
    }
}

implement_tile!(StairDown);
