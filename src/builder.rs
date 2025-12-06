pub trait Builder<T> {
    fn new_from(base: &T) -> Self;

    fn build(&self) -> T;
}