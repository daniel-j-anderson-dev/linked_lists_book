/// Naive linked list wont compile
#[derive(Debug)]
pub enum Node<T> {
    Nil,
    Construct(T, Box<Node<T>>),
}
impl<T> Node<T> {
    pub fn cons(element: T, next: Box<Self>) -> Box<Self> {
        return Box::new(Node::Construct(element, next));
    }
    pub fn nil() -> Box<Self> {
        return Box::new(Node::Nil);
    }
}

#[test]
fn list() {
    let list = Node::cons(0, Node::cons(1, Node::cons(2, Node::nil())));
    println!("{:?}", list);
}