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

    fn inorder_traversal(&self) -> Vec<T> {
        todo!()
    }

    fn preorder_traversal(&self) -> Vec<T> {
        todo!()
    }

    fn postorder_traversal(&self) -> Vec<T> {
        todo!()
    }

    fn level_order_traversal(&self) -> Vec<T> {
        todo!()
    }
}
