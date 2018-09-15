pub trait GameObject {
    fn get_position(&self) -> (i32, i32);

    fn move_object(&self, position: (i32, i32));
}