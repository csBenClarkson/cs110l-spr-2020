use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T> fmt::Display for LinkedList<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        print!("|");
        loop {
            match current {
                Some(node) => {
                    result = format!("{} <- {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T> Clone for Node<T> where T: Clone {
    fn clone(&self) -> Self {
        Node::new(self.value.clone(), self.next.clone())
    }
}

impl<T> Clone for LinkedList<T> where T: Clone {
    fn clone(&self) -> Self {
        // recursive solution:
        LinkedList { head: self.head.clone(), size: self.size }

        // iterative solution (not finished):

        // if self.head.is_none() { return LinkedList::new() }       // empty list
        // let mut new_head = Box::new(Node::new(self.head.as_ref().unwrap().value.clone(), None));
        // let mut current = &mut new_head;
        // let mut next = &self.head.as_ref().unwrap().next;
        // loop {
        //     match next {
        //         Some(this) => {
        //             current.next = Some(Box::new(Node::new(this.value.clone(), None)));
        //             next = &this.next;
        //             current = &mut current.next.unwrap();          // must not be None
        //         },
        //         None => break,
        //     }
        // }
        // LinkedList { head: Some(new_head), size: self.size}
    }
}

impl<T> PartialEq for LinkedList<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size { return false; }
        let mut node_self = &self.head;
        let mut node_other = &other.head;
        while node_self.is_some() {
            if node_self.as_ref().unwrap().value != node_other.as_ref().unwrap().value {
                return false;
            }
            node_self = &node_self.as_ref().unwrap().next;
            node_other = &node_other.as_ref().unwrap().next;
        }
        return true;
    }
}

pub struct LinkedListIterator<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIterator<'_, T> where T: Clone {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value.clone())
            }
            None => { None }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> where T: Clone {
    type Item = T;
    type IntoIter = LinkedListIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator { current: &self.head }
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64;
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        let mut sum = 0.0;
        for v in self {
            sum += v.powi(2);
        }
        sum.sqrt()
    }
}


