pub trait Builder<T>: From<T> + Into<T> {
    fn clear(&mut self);
    fn build(&self) -> T;
}