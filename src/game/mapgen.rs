use game::map::tile::Tile;
use game::map::tile::Wall;

pub fn dummy_gen() -> Vec<Box<Tile>> {
    let mut ret: Vec<Box<Tile>> = Vec::new();
    ret.push(Box::new(Wall::new(20, 20)));

    return ret;
}
