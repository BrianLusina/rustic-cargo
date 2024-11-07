pub(crate) trait CustomIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
