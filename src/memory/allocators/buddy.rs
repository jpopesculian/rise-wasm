use super::{Allocator, AllocatorRef};
use crate::utils::errors::RuntimeError;
use alloc::prelude::*;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp::max;
use hashbrown::HashMap;

const BASE_ORDER: u8 = 8;
const MIN_ORDER: u8 = 4; // 4096

#[derive(Debug, PartialEq)]
pub struct BuddyAllocator {
    heap_base: u32,
    node_list: NodeRef,
    max_order: u8,
}

impl BuddyAllocator {
    pub fn new(heap_base: u32, max_offset: u32) -> Result<BuddyAllocator, RuntimeError> {
        let size = max_offset - heap_base;
        let max_order = largest_order(size) - BASE_ORDER;
        if max_order < MIN_ORDER {
            return Err(RuntimeError::new("Heap size is too small"));
        }
        Ok(BuddyAllocator {
            heap_base,
            node_list: NodeRef::new(0, max_order),
            max_order,
        })
    }

    pub fn find_free_node(&mut self, size: u32) -> Option<NodeRef> {
        let order = self.size_to_order(size);
        let mut node = self.node_list.clone();
        while !node.is_free() || node.get_order() != order {
            if node.is_free() {
                node.split();
            } else if let Some(next) = node.next() {
                node = next.clone();
            } else {
                return None;
            }
        }
        Some(node)
    }

    pub fn size_to_order(&self, size: u32) -> u8 {
        self.max_order - (smallest_order(size) - BASE_ORDER)
    }

    pub fn order_to_size(&self, order: u8) -> u32 {
        1 << (self.max_order - order + BASE_ORDER)
    }
}

fn smallest_order(n: u32) -> u8 {
    (n as f64).log2().ceil() as u8
}

fn largest_order(n: u32) -> u8 {
    (n as f64).log2().floor() as u8
}

#[derive(Clone, Debug, PartialEq)]
struct NodeRef(Rc<RefCell<Node>>);

#[derive(Debug, PartialEq)]
struct Node {
    pub next: Option<NodeRef>,
    pub prev: Option<NodeRef>,
    pub buddies: HashMap<u8, NodeRef>,
    pub free: bool,
    pub order: u8,
    pub max_order: u8,
    pub id: u32,
}

impl NodeRef {
    pub fn new(order: u8, max_order: u8) -> NodeRef {
        NodeRef(Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            buddies: HashMap::new(),
            free: true,
            id: 0,
            order,
            max_order,
        })))
    }

    pub fn next(&self) -> Option<NodeRef> {
        self.0.borrow().next.clone()
    }

    pub fn prev(&self) -> Option<NodeRef> {
        self.0.borrow().prev.clone()
    }

    pub fn set_next(&self, next: Option<NodeRef>) {
        self.0.borrow_mut().next = next
    }

    pub fn set_prev(&self, prev: Option<NodeRef>) {
        self.0.borrow_mut().prev = prev
    }

    pub fn get_buddy(&self) -> Option<NodeRef> {
        self.0
            .borrow()
            .buddies
            .get(&self.get_order())
            .map(Clone::clone)
    }

    pub fn insert_buddy(&self, order: u8, buddy: NodeRef) {
        let _ = self.0.borrow_mut().buddies.insert(order, buddy);
    }

    pub fn is_free(&self) -> bool {
        self.0.borrow().free
    }

    pub fn free(&self) {
        self.0.borrow_mut().free = true
    }

    pub fn claim(&self) {
        self.0.borrow_mut().free = false
    }

    pub fn max_order(&self) -> u8 {
        self.0.borrow().max_order
    }

    pub fn get_order(&self) -> u8 {
        self.0.borrow().order
    }

    pub fn set_order(&self, order: u8) {
        self.0.borrow_mut().order = order
    }

    pub fn set_id(&self, id: u32) {
        self.0.borrow_mut().id = id
    }

    pub fn get_id(&self) -> u32 {
        self.0.borrow().id
    }

    pub fn tree_string(&self) -> String {
        let mut result = String::from("");
        let mut node = self.clone();
        while node.prev() != None {
            node = node.prev().unwrap();
        }
        result = format!("{}, {}", result, node.to_string());
        while node.next() != None {
            node = node.next().unwrap();
            result = format!("{}, {}", result, node.to_string());
        }
        result
    }

    pub fn to_string(&self) -> String {
        format!("Node<id: {}, order: {}>", self.get_id(), self.get_order())
    }

    pub fn split(&self) {
        let new_order = self.get_order() + 1;
        let buddy_id = self.get_id() | 1 << (self.max_order() - new_order);
        let buddy = NodeRef::new(new_order, self.max_order());

        buddy.set_next(self.next());
        buddy.set_id(buddy_id);

        self.set_next(Some(buddy.clone()));
        buddy.set_prev(Some(self.clone()));

        for (k, b) in self.0.borrow().buddies.iter() {
            buddy.insert_buddy(*k, b.clone());
        }
        self.insert_buddy(new_order, buddy.clone());
        buddy.insert_buddy(new_order, self.clone());

        self.set_order(new_order);
        buddy.set_order(new_order);
    }

    pub fn merge(&self) {
        let old_order = self.get_order();
        let new_id = self.get_id() & !(1 << (self.max_order() - old_order));
        let new_order = old_order - 1;
        let mut next = self.next();
        let mut prev = self.prev();
        if let Some(buddy) = self.0.borrow().buddies.get(&self.get_order()) {
            if buddy.get_id() > self.get_id() {
                next = buddy.next();
            } else {
                prev = buddy.prev();
            }
        }
        if let Some(buddy) = self.0.borrow().buddies.get(&new_order) {
            buddy.insert_buddy(new_order, self.clone());
        }
        self.set_id(new_id);
        self.set_order(new_order);
        self.set_next(next.clone());
        self.set_prev(prev.clone());
        if let Some(node) = prev {
            node.set_next(Some(self.clone()))
        }
        if let Some(node) = next {
            node.set_prev(Some(self.clone()))
        }
        self.0.borrow_mut().buddies.get(&old_order);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_largest_order() {
        assert_eq!(largest_order(0), 0);
        assert_eq!(largest_order(1), 0);
        assert_eq!(largest_order(2), 1);
        assert_eq!(largest_order(10), 3);
        assert_eq!(largest_order(8), 3);
        assert_eq!(largest_order(130), 7);
    }

    #[test]
    fn test_smallest_order() {
        assert_eq!(smallest_order(0), 0);
        assert_eq!(smallest_order(1), 0);
        assert_eq!(smallest_order(2), 1);
        assert_eq!(smallest_order(10), 4);
        assert_eq!(smallest_order(8), 3);
        assert_eq!(smallest_order(130), 8);
    }

    #[test]
    fn node_ref_split() {
        let node = NodeRef::new(0, 5);
        node.split();
        assert_eq!(node.get_order(), 1);
        assert_ne!(node.next(), None);
        assert_eq!(node.prev(), None);
        assert_eq!(node.get_id(), 0b0);

        let buddy = node.get_buddy().unwrap();
        assert_ne!(buddy.get_buddy(), None);
        assert_eq!(buddy.next(), None);
        assert_ne!(buddy.prev(), None);
        assert_eq!(buddy.get_id(), 0b10000);

        node.split();
        assert_eq!(node.get_order(), 2);

        let new_buddy = node.get_buddy().unwrap();
        assert_eq!(new_buddy.get_order(), 2);
        assert_eq!(new_buddy.get_id(), 0b1000);

        assert_eq!(buddy.get_buddy().unwrap().get_order(), 2);
        assert_eq!(buddy.get_order(), 1);
    }

    #[test]
    fn node_ref_merge() {
        let node = NodeRef::new(0, 5);
        node.split();
        node.split();
        let buddy = node.get_buddy().unwrap();
        buddy.split();
        buddy.split();
        buddy.merge();
        buddy.get_buddy().unwrap().merge();
        node.merge();
        panic!("{}", node.tree_string());
    }

    #[test]
    fn allocator_new() {
        let allocator = BuddyAllocator::new(1000, 10000).unwrap();
        assert_eq!(
            allocator,
            BuddyAllocator {
                heap_base: 1000,
                node_list: NodeRef::new(0, 5),
                max_order: 5
            }
        );
    }

    #[test]
    fn find_free_node() {
        let mut allocator = BuddyAllocator::new(1000, 10000).unwrap();
        let node = allocator.find_free_node(400);
    }

    #[test]
    fn bitshift() {}
}
