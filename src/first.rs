pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List {head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head,
        };
    }
}

enum Link{
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}

