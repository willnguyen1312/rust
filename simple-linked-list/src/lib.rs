use std::iter::FromIterator;

pub struct SimpleLinkedList<T>(Vec<T>);

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, _element: T) {
        self.0.push(_element)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut v = self.0;
        v.reverse();
        SimpleLinkedList(v)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let v = Vec::from_iter(_iter);
        SimpleLinkedList(v)
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list.0
    }
}
