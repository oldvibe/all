// pub struct Field {
//     head: Link,
// }

// type Link = Option<Box<Node>>;

// struct Node {
//     elem: Target,
//     next: Link,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct Target {
//     pub size: u32,
//     pub xp: u32,
// }

// impl Field {
//     pub fn new() -> Self {
//         Field {
//             head: None,
//         }
//     }
//     pub fn push(&mut self, target: Target) {
//         self.head = Some(Box::new(Node{elem: target,next: self.head.take(),}))
//     }
//     pub fn pop(&mut self) -> Option<Target> {
//         if let Some(h) = &self.head{
//             let b = h.elem.clone();
//             self.head = h.next.clone();
//             Some(b)
//         }else{
//             None
//         }
//     }
//     pub fn peek(&self) -> Option<&Target> {}
//     pub fn peek_mut(&mut self) -> Option<&mut Target> {}
// }
pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Clone)]
struct Node {
    elem: Target,
    next: Link,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}
impl Field {
    pub fn new() -> Self {
        Field { head: None }
    }
    pub fn push(&mut self, target: Target) {
        self.head = Some(Box::new(Node {
            elem: target,
            next: self.head.take(),
        }))
    }
    pub fn pop(&mut self) -> Option<Target> {
        if let Some(h) = &self.head {
            let b = h.elem.clone();
            self.head = h.next.clone();
            Some(b)
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<&Target> {
        if let Some(h) = &self.head {
            Some(&h.elem)
        } else {
            None
        }
    }
    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        if let Some(h) = &mut self.head {
            Some(&mut h.elem)
        } else {
            None
        }
    }
}