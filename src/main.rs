// Linked list in rust, as recommended by exo
// no tail reference stored in the LL struct as Box is a unique smart ptr

use std::fmt::Display;


struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: i32,
}

impl<T: Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.insert_node(Box::new(Node {val: value, next: None}));
    }

    fn insert_node(&mut self, node:Box<Node<T>>) {
        // if empty, create a new node as the head
        if self.length == 0 {
            self.head = Some(node);
            self.length += 1;
            return;
        }

        //otherwise, 
        let mut curr = self.head.as_mut().unwrap();
        loop {
            if curr.next.is_some() {
                curr = curr.next.as_mut().unwrap();
            } else {
                curr.next = Some(node);
                self.length += 1;
                break;
            }
        }
    }

    pub fn print(&self) {
        if self.length == 0 {
            return;
        }

        let mut curr = self.head.as_ref().unwrap();
        print!("List[{}, ", curr.val);
        loop {
            if curr.next.is_some() {
                curr = curr.next.as_ref().unwrap();
                print!("{}, ", curr.val);
                
            } else {
                println!("]");
                break;
            }
        }
    }

}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}



fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.insert(5);
    list.insert(10);
    list.print();
    println!("length is {}", list.length);
}
