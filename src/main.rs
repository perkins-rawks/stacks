/*
* Authors: Awildo G., Sam. A., Siqi F., Sosina A.
* Date: May 27, 2020
* Project: Stack Data Structure
* Desc: We implement a stack using vectors.
*/

use std::fmt::Display;

// We implement Stack for displayable items
pub struct Stack<T: Display> {
    // The top of the stack is the last item in contents
    size: usize,
    contents: Vec<T>,
}

impl<T: Display> Stack<T> {
    // Constructor of Objects of type Stack.
    pub fn new() -> Self {
        Self {
            size: 0,
            contents: Vec::new(),
        }
    }

    // Pushes an item onto the top of the Stack
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
        self.size += 1;
    }

    // Pop removes the top of the Stack
    pub fn pop(&mut self) -> T {
        assert!(!self.empty());
        self.size -= 1;
        self.contents.pop().unwrap()
    }

    // Returns the size of the Stack
    pub fn size(&self) -> usize {
        self.size
    }

    // Returns true when empty, false otherwise
    pub fn empty(&self) -> bool {
        self.size == 0
    }

    // Clears the stack and its contents
    pub fn clear(&mut self) {
        self.contents.clear();
        self.size = 0;
    }
    // Prints the Stack from the bottom to top.
    pub fn print(&self) {
        if self.empty() {
            println!("[]");
            return;
        }

        println!("From bottom to top [Bottom ... Top], the Stack contains: ");
        print!("[");
        for (idx, item) in self.contents.iter().enumerate() {
            if idx < self.size - 1 {
                print!("{}, ", item);
            } else {
                println!("{}]", item);
            }
        }
    }
}

fn main() {
    println!("------------Test 1------------");
    let mut test1: Stack<&str> = Stack::new();
    test1.push("World!");
    test1.push(", ");
    test1.push("Hello");
    test1.print();
    println!("Size: {}", test1.size());
    println!("Empty?: {}", if test1.empty() { "Yes!" } else { "No!" });
    test1.clear();
    println!("Cleared");
    test1.push("World!");
    test1.push(", ");
    test1.push("Hello");
    print!("{}", test1.pop());
    print!("{}", test1.pop());
    println!("{}", test1.pop());

    println!("------------Test 2------------");

    let mut test2: Stack<u32> = Stack::new();

    print!("Test 2 is empty (T/F): {}", test2.empty());
    for i in 0..15 {
        test2.push(i);
        test2.print();
    }
    for _j in 0..15 {
        test2.pop();
        test2.print();
    }
}
