pub trait State<T> {
    fn get_state(&mut self) -> &mut T;
}