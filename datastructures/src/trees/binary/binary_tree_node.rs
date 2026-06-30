use crate::trees::tree_node::TreeNode;

#[derive(Debug)]
pub struct BinaryTreeNode<T> {
    data: T,
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    // TODO: set new to enable adding optional left and right nodes
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    fn set_left(&mut self, node: BinaryTreeNode<T>) {
        self.left = Option::from(Box::from(node))
    }

    fn set_right(&mut self, node: BinaryTreeNode<T>) {
        self.right = Option::from(Box::from(node))
    }
}

impl<T> TreeNode<T> for BinaryTreeNode<T> {
    fn data(&self) -> T {
        todo!()
    }
}
