// use tcod::input::Key;
// use tcod::console::*;

#[derive(Debug)]
pub struct Wall{
    pub x: i32,
    pub y: i32
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