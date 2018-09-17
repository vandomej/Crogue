pub trait GameObject {
    fn get_position(&self) -> (i32, i32);

    fn move_object(&self, position: (i32, i32));

    fn is_adjacent_to<T>(&self, other: &T) -> bool
        where T: GameObject
    {
        let (x1, y1) = self.get_position();
        let (x2, y2) = other.get_position();

        return (x2 <= x1 + 1) && (x2 >= x1 - 1) && (y2 <= y1 + 1) && (y2 >= y1 - 1) 
    }
}