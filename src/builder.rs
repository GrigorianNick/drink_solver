pub trait Builder<T>: From<T> {
    fn clear(&mut self);
    fn build(&self) -> T;
}
