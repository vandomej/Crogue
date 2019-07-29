use tcod::console::*;

pub enum SceneTransitionType {
    Up,
    Down
}

pub trait Tile {
    fn new(x: i32, y: i32) -> Self where Self: Sized;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn get_walkable(&self) -> bool;
    fn get_see_through(&self) -> bool;
    fn draw(&self, window: &Root);
    fn clear(&self, window: &Root);
    fn causes_scene_transitions(&self) -> &Option<SceneTransitionType>;
}


pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Tile for Wall {
    fn new(x: i32, y: i32) -> Wall {
        return Wall{x, y, walkable: false, see_through: false, causes_scene_transitions: None};
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

    fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }

    fn causes_scene_transitions(&self) -> &Option<SceneTransitionType> {
        return &self.causes_scene_transitions;
    }
}


pub struct Floor {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Tile for Floor {
    fn new(x: i32, y: i32) -> Floor {
        return Floor{x, y, walkable: true, see_through: true, causes_scene_transitions: None};
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

    fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }

    fn causes_scene_transitions(&self) -> &Option<SceneTransitionType> {
        return &self.causes_scene_transitions;
    }
}

pub struct StairUp {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Tile for StairUp {
    fn new(x: i32, y: i32) -> StairUp {
        return StairUp{x, y, walkable: true, see_through: true, causes_scene_transitions: Some(SceneTransitionType::Up)};
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
        window.put_char(self.x, self.y, '<', BackgroundFlag::Set);
    }

    fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }

    fn causes_scene_transitions(&self) -> &Option<SceneTransitionType> {
        return &self.causes_scene_transitions;
    }
}

pub struct StairDown {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub see_through: bool,
    pub causes_scene_transitions: Option<SceneTransitionType>
}

impl Tile for StairDown {
    fn new(x: i32, y: i32) -> StairDown {
        return StairDown{x, y, walkable: true, see_through: true, causes_scene_transitions: Some(SceneTransitionType::Down)};
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
        window.put_char(self.x, self.y, '>', BackgroundFlag::Set);
    }

    fn clear(&self, mut window: &Root) {
        window.put_char(self.x, self.y, ' ', BackgroundFlag::Set);
    }

    fn causes_scene_transitions(&self) -> &Option<SceneTransitionType> {
        return &self.causes_scene_transitions;
    }
}