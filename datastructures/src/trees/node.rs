pub trait Node<T> {
    fn left(&self) -> T;
    fn right(&self) -> T;
    fn data(&self) -> T;
}
