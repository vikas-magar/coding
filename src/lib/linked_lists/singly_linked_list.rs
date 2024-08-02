/// Starting with linked list
/// Learning linked list
/// ```assert_eq(1, 1)```

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_one() {
        let input = vec![1, 4, 2, 3];
        let mut ll = List::new();
        assert_eq!(ll.pop(), None);
        for ele in input {
            ll.push(ele);
        }

        assert_eq!(ll.pop(), Some(3));
    }

    #[test]
    fn peek() {
        let mut ll = List::new();
        assert_eq!(ll.peek(), None);
        assert_eq!(ll.peek_mut(), None);
        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.peek(), Some(&3));
        assert_eq!(ll.peek_mut(), Some(&mut 3));

        ll.peek_mut().map(|value| *value = 42);

        assert_eq!(ll.peek(), Some(&42));
        assert_eq!(ll.pop(), Some(42));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
