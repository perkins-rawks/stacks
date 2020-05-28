/*
* Authors: Awildo G., Sam. A., Siqi F., Sosina A.
* Date: May 27, 2020
* Project: Stack Data Structure
* Desc: We implement a stack using vectors.
*/

use std::fmt::Display;
use std::marker::Copy; // we want to be able to return copies of objects

// We implement Stack for displayable and copy-able items
pub struct Stack<T: Display + Copy> {
    // The top of the stack is the last item in contents
    size: usize,
    contents: Vec<T>,
}

impl<T: Display + Copy> Stack<T> {
    // Constructor of Objects of type Stack.
    pub fn new() -> Self {
        Self {
            size: 0,
            contents: Vec::new(),
        }
    }

    // Returns the top element of the Stack
    pub fn top(&self) -> T {
        assert!(!self.empty());
        self.contents[self.size - 1]
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

        // deletes the top and returns the top
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
    println!("------------Test 1-------------");
    let mut test1: Stack<&str> = Stack::new();
    //println!("{}", test1.top());
    test1.push("1");
    println!("Top: (Should be 1) {}", test1.top());
    test1.push("2");
    test1.push("3");
    test1.print();
    println!("Size: (Should be 3) {}", test1.size());
}
