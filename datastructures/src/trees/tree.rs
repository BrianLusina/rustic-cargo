pub trait Tree<T> {
    fn root(&self) -> T;
    
    /// inorder traversal
    fn inorder_traversal(&self) -> Vec<T>;
    
    /// Preorder traversal
    fn preorder_traversal(&self) -> Vec<T>;
    
    /// Postorder traversal
    fn postorder_traversal(&self) -> Vec<T>;
    
    /// Level order traversal
    fn level_order_traversal(&self) -> Vec<T>;
}
