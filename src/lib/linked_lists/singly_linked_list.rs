use std::mem;
/// Starting with linked list
/// Learning linked list
/// ```assert_eq(1, 1)```

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
}

fn run(inp: &Vec<i32>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = vec![1, 4, 2, 3];
        let opt = run(&input);
        assert_eq!(1, 1)
    }
}
