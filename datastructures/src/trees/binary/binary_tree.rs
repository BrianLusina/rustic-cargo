use crate::trees::tree::Tree;

#[derive(Debug)]
pub struct BinaryTree {}

impl BinaryTree {
    fn new() -> Self {
        Self {}
    }
}

impl<T> Tree<T> for BinaryTree {
    fn root(&self) -> T {
        todo!()
    }
}
