/// Naive linked list wont compile
pub enum List<T> {
    Empty,
    Element(T, List<T>),
}