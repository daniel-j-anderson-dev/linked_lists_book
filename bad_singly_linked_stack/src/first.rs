/// Naive linked list
pub type Next<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Next<T>
}
impl<T> Node<T> {
    pub fn cons(element: T, next: Next<T>) -> Next<T> {
        return Some(Box::new(Node { element, next }));
    }
    pub fn nil() -> Next<T> {
        return None;
    }
}

#[test]
fn list() {
    let list = Node::cons(0, Node::cons(1, Node::cons(2, Node::nil())));
    println!("{:?}", list);
}
