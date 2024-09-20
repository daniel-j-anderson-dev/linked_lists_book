pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub const fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        // take the current head off the list.
        let old_head = self.head.take();

        // create the new head specifying the old head will come next
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });

        // set the current head of the list to the new head
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        // take the current head off the list
        let popped_node = self.head.take()?;

        // set the current head to the Node after the popped_node
        self.head = popped_node.next;

        // return the element of the popped node
        Some(popped_node.element)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // start by taking off the head
        let mut current_link = self.head.take();

        loop {
            match current_link {
                Some(mut current_node) => {
                    // take the next node
                    current_link = current_node.next.take()

                    // `current_node` is dropped.
                    // Since `current_node` doesn't have a next node anymore
                    // there is no unbounded recursion when `current_node` is dropped.
                }
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn drop_large_list() {
        let mut list = List::<usize>::new();

        for i in 0..100000 {
            list.push(i);
        }

        drop(list);
    }
}

