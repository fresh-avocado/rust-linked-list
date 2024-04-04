use std::fmt::Display;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Default> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            next: None,
        }
    }
}

struct LinkedList<T> {
    head: Box<Node<T>>,
}

impl<T: Default + Display> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: Box::new(Node::new(T::default()))
        }
    }

    fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        // get mutable pointer to head
        let mut current = &mut self.head.next;
        // Rust's `while (true)`
        loop {
            match current {
                None => {
                    *current = Some(new_node);
                    break;
                },
                Some(next_node) => {
                    // `&var` gets a read-only pointer to `var`
                    // `&mut var` gets a read-write pointer to `var`
                    current = &mut next_node.next;
                },
            }
        }
    }

    fn print(&self) {
        // let curr = self.head; // moves `self.head` into `curr`
        // let curr = &self.head; // `curr` is an immutable ptr to `self.head`
        let mut curr = &self.head.next;
        if curr.is_none() {
            println!("Linked list is empty...");
            return;
        }
        loop {
            match curr {
                None => {
                    println!("\x08 \x08\x08 \x08");
                    break;
                },
                Some(node) => {
                    print!("{}, ", node.data);
                    curr = &node.next;
                },
            }
        }
    }
}

fn main() {
    let mut ll: LinkedList<String> = LinkedList::new();
    ll.print();
    ll.push_back(String::from("heyy"));
    ll.print();
    ll.push_back(String::from("wassup?"));
    ll.print();
}
