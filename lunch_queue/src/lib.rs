// pub struct Queue {
//     pub node: Link,
// }

// pub type Link =

// pub struct Person {
//     pub discount: i32,
//     pub name: String,
// }

// impl Queue {
//     pub fn new() -> Queue {

//     }
//     pub fn add(&mut self, name: String, discount: i32) {

//     }
//     pub fn invert_queue(&mut self) {

//     }
//     pub fn rm(&mut self) -> Option<(String, i32)> {

//     }
//     pub fn search(&self, name: &str) -> Option<(String, i32)> {

//     }
// }
use std::{env::current_exe, thread::current};

#[derive(Debug, Clone, PartialEq)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;
#[derive(Debug, Clone, PartialEq)]

pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{node: None}
    }
    pub fn add(&mut self, name: String, discount: i32) {
        let new_node = Person {
            discount,
            name,
            next: self.node.take(),
        };

        self.node = Some(Box::new(new_node));
    }
    pub fn invert_queue(&mut self) {
        let mut new_q = Queue{node: None};
        let mut current = self.node.take();
        while let Some(ref node) =  current {
            new_q.add(node.name.clone(), node.discount.clone());
            current = current.unwrap().next;
        }
        *self = new_q
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        self.invert_queue();

        let node = self.node.clone();
        self.node = self.node.clone().unwrap().next;
        self.invert_queue();
       Some((node.clone().unwrap().name, node.clone().unwrap().discount))
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = self.node.clone();
        while let Some(ref node) = current {
            if node.name == name {
                return Some((node.clone().name, node.clone().discount));
            }
            current = current.unwrap().next;
        }
        None
    }
}