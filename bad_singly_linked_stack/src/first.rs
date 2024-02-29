/// Naive linked list
#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn cons(element: T, next: Option<Box<Self>>) -> Option<Box<Self>> {
        return Some(Box::new(Node(element, next)));
    }
    pub fn nil() -> Option<Box<Self>> {
        return None;
    }
}

#[test]
fn list() {
    let list = Node::cons(0, Node::cons(1, Node::cons(2, Node::nil())));
    println!("{:?}", list);
}