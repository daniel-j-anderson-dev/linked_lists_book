use std::mem;

pub struct List<E> {
    head: Link<E>,
}

enum Link<E> {
    Empty,
    More(Box<Node<E>>),
}

struct Node<E> {
    element: E,
    next: Link<E>,
}

impl<E> List<E> {
    pub const fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, element: E) {
        // take the current head off the list.
        let old_head = mem::replace(&mut self.head, Link::Empty);

        // create the new head specifying the old head will come next
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });

        // set the current head of the list to the new head
        self.head = Link::More(new_head);
    }

    pub fn pop(&mut self) -> Option<E> {
        // take the current head off the list
        let popped_node = match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => return None,
            Link::More(node) => node,
        };

        // set the current head to the Node after the popped_node
        self.head = popped_node.next;

        // return the element of the popped node
        Some(popped_node.element)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // start by taking off the head
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        loop {
            match current_link {
                Link::More(mut current_node) => {
                    // take the next node
                    current_link = mem::replace(&mut current_node.next, Link::Empty)

                    // `current_node` is dropped.
                    // Since `current_node` doesn't have a next node anymore
                    // there is no unbounded recursion when `current_node` is dropped.
                }
                Link::Empty => break,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn drop_large_list() {
        let mut list = List::new();

        for i in 0..100000 {
            list.push(i);
        }

        drop(list);
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::new();

        for i in 0..100 {
            list.push(i);
        }

        for i in (0..100).rev() {
            assert_eq!(Some(i), list.pop());
        }
    }
}
