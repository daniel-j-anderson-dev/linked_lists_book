pub mod iter;

pub struct List<E> {
    head: Link<E>,
}

type Link<E> = Option<Box<Node<E>>>;

struct Node<E> {
    element: E,
    next: Link<E>,
}

impl<E> List<E> {
    pub const fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: E) {
        self.head = Some(Box::new(Node {
            element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<E> {
        self.head.take().map(|popped_node| {
            self.head = popped_node.next;
            popped_node.element
        })
    }

    pub fn peek(&self) -> Option<&E> {
        self.head.as_ref().map(|node| &node.element)
    }
    pub fn peek_mut(&mut self) -> Option<&mut E> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        loop {
            match current_link {
                Some(mut current_node) => {
                    current_link = current_node.next.take();
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

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(None, list.peek());
        assert_eq!(None, list.peek_mut());

        for i in 0..100 {
            list.push(i);
        }

        assert_eq!(Some(&99), list.peek());
        assert_eq!(Some(99), list.pop());

        assert_eq!(Some(&mut 98), list.peek_mut());
        list.peek_mut().map(|element| *element = -1);
        assert_eq!(Some(-1), list.pop());
    }


    #[test]
    fn iteration() {
        let original_values = ['a', 'b', 'c'];
        let mutated_values = ['1', '2', '3'];

        let mut list = List::new();
        list.push('c');
        list.push('b');
        list.push('a');

        for (i, element) in list.iter().enumerate() {
            assert_eq!(*element, original_values[i]);
        }

        for (i, element) in list.iter_mut().enumerate() {
            *element = mutated_values[i];
        }

        for (i, mut element) in list.into_iter().enumerate() {
            assert_eq!(element, mutated_values[i]);
            
            element = '\0';

            assert_eq!(element, '\0');
        }
    }
}
