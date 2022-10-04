// https://youtu.be/ygL_xcavzQ4?t=7551
fn main() {
    println!("Hello, world!");
    // Box

    let b_int1 = Box::new(10);

    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl <T> TreeNode<T>{
        pub fn new(key: T) -> Self {
            TreeNode{left: None, right:None, key}
        }


        pub fn left (mut self, node: TreeNode<T>)->Self{
            self.left = Some(Box::new(node));
            self
        }

        pub fn right (mut self, node: TreeNode<T>)->Self{
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

    // I think that I need to return to the topic of Box and smart pointers
    // it is not totally clear to me how and why do we use them.

}
