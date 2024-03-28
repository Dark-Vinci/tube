use std::{
    ptr,
    fmt,
};

struct Node<T> {
    data: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(d: T) -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            data: d,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T: fmt::Debug> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    fn len(&self) -> usize {
        let mut current = self.head;

        let mut count = 0;

        while !current.is_null() {
            count += 1;

            unsafe {
                current = (*current).next
            }
        }

        return count;
    }

    fn push_front(&mut self, d: T) {
        let mut new_node = Node::new(d);

        if self.head.is_null() {
            self.head = &mut new_node as *mut Node<T>;
            self.tail = self.head;
        } else {
            new_node.next = self.head;

            unsafe {
                (*self.head).prev = &mut new_node as *mut Node<T>;
            }

            self.head = &mut new_node as *mut Node<T>;
        }
    }

    fn push_back(&mut self, d: T) {
        let mut new_node = Node::new(d);

        if self.tail.is_null() {
            self.tail = &mut new_node as *mut Node<T>;
            self.head = self.tail;
        } else {
            new_node.prev = self.tail;

            unsafe {
                (*self.tail).next = &mut new_node as *mut Node<T>;
            }

            self.tail = &mut new_node as *mut Node<T>;
        }
    }

    fn print(&self) {
        unsafe {
            let mut current = self.head;

            while !current.is_null() {
                let node_ref = &*current;

                let data = &((*node_ref).data);
                println!("The data| {:?}", data);

                // Move to the next node
                current = (*current).next;
            }
        }
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // Insert elements into the list
    // list.push_front(1);
    list.push_front(2);
    // list.push_front(3);

    // Print the elements of the list
    list.print();
    println!("Try program.pro");
}
