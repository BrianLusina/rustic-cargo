use crate::trees::tree_node::TreeNode;

#[derive(Debug)]
pub struct BinaryTreeNode<T> {
    data: T,
    left: Box<BinaryTreeNode<T>>,
    right: Box<BinaryTreeNode<T>>,
}

impl<T> BinaryTreeNode<T> {
    // TODO: set new
    // fn new(data: T) -> Self {
    //     Self {
    //         data,
    //         left: Box::new(BinaryTreeNode {}),
    //         right: Box::new(BinaryTreeNode {}),
    //     }
    // }

    fn set_left(&mut self, node: BinaryTreeNode<T>) {
        self.left = Box::from(node)
    }

    fn set_right(&mut self, node: BinaryTreeNode<T>) {
        self.right = Box::from(node)
    }
}

impl<T> TreeNode<T> for BinaryTreeNode<T> {
    fn data(&self) -> T {
        todo!()
    }
}
