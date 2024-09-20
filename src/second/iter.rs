use super::{List, Node};

impl<E> IntoIterator for List<E> {
    type Item = E;
    type IntoIter = ValueIterator<E>;

    fn into_iter(self) -> Self::IntoIter {
        ValueIterator(self)
    }
}
impl<E> List<E> {
    pub fn iter(&self) -> ReferenceIterator<'_, E> {
        ReferenceIterator { next: self.head.as_deref() }
    }
    pub fn iter_mut(&mut self) -> MutableReferenceIterator<E> {
        MutableReferenceIterator { next: self.head.as_deref_mut() }
    }
}

pub struct ValueIterator<E>(List<E>);
impl<E> Iterator for ValueIterator<E> {
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct ReferenceIterator<'a, E> {
    next: Option<&'a Node<E>>,
}
impl<'a, E> Iterator for ReferenceIterator<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        // next is not moved because a reference is Copy
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct MutableReferenceIterator<'a, E> {
    next: Option<&'a mut Node<E>>
}
impl<'a, E> Iterator for MutableReferenceIterator<'a, E> {
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        // since mutable reference is NOT Copy
        // we have to `take` the mutable reference
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}
