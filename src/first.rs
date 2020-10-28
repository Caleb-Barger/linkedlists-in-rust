use std::mem; // indiana jones shit

pub struct List {
    // allows for hidden impl details
    head: Link,
}

impl List {
    // implements list methods
    pub fn new() -> Self {
        // note the capital S
        List { head: Link::Empty } // Initialize a new list ez
    }

    pub fn push(&mut self, elem: i32) {
        // mutable ref to self
        let new_node = Box::new(Node {
            // Recursive Data structures need a box
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), // Indiana jones Temporarily
            // swaps next with Empty and then back to the head of the list
        });

        self.head = Link::More(new_node) // set head as the new node
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}
