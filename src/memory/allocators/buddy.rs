use super::{Allocator, AllocatorRef};
use crate::utils::errors::RuntimeError;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp::{max, PartialEq};
use core::fmt;
use core::iter::Iterator;
use hashbrown::HashMap;

const BASE_ORDER: u8 = 7; // 128
const MIN_ORDER: u8 = 4; // 4096

#[derive(Debug, PartialEq)]
pub struct BuddyAllocator {
    heap_base: u32,
    node_list: NodeRef,
    max_order: u8,
}

impl Allocator for BuddyAllocator {
    fn allocate(&mut self, size: u32) -> Result<u32, RuntimeError> {
        if let Some(node) = self.find_free_node(size) {
            node.claim();
            Ok(self.get_offset_by_node_id(node.get_id()))
        } else {
            Err(RuntimeError::new("Heap out of space"))
        }
    }

    fn free(&mut self, ptr: u32) -> Result<(), RuntimeError> {
        if let Some(node) = self.find_node_by_offset(ptr) {
            node.free();
            self.recursive_merge(&node);
            Ok(())
        } else {
            Err(RuntimeError::new("Invalid pointer"))
        }
    }

    fn reset(&mut self) -> Result<(), RuntimeError> {
        self.node_list = NodeRef::new(0, self.max_order);
        Ok(())
    }
}

impl BuddyAllocator {
    pub fn new(heap_base: u32, max_offset: u32) -> Result<AllocatorRef, RuntimeError> {
        let size = max_offset - heap_base;
        let max_order = largest_order(size) - BASE_ORDER;
        if max_order < MIN_ORDER {
            return Err(RuntimeError::new("Heap size is too small"));
        }
        Ok(AllocatorRef(Rc::new(RefCell::new(BuddyAllocator {
            heap_base,
            node_list: NodeRef::new(0, max_order),
            max_order,
        }))))
    }

    fn find_free_node(&mut self, size: u32) -> Option<NodeRef> {
        let order = self.size_to_order(size);
        let mut node = self.node_list.clone();
        while !node.is_free() || node.get_order() != order {
            if node.is_free() && node.get_order() < order {
                node.split();
            } else if let Some(next) = node.next() {
                node = next.clone();
            } else {
                return None;
            }
        }
        Some(node)
    }

    fn recursive_merge(&mut self, node: &NodeRef) {
        let mut buddy = node.get_buddy();
        while {
            if let Some(buddy) = buddy.clone() {
                buddy.is_free() && buddy.get_order() == node.get_order()
            } else {
                false
            }
        } {
            node.merge();
            buddy = node.get_buddy();
        }
    }

    fn find_node_by_offset(&self, offset: u32) -> Option<NodeRef> {
        if !self.valid_offset(offset) {
            return None;
        }
        let id = self.get_node_id_by_offset(offset);
        for node in self.node_list.iter() {
            let node_id = node.get_id();
            if node_id == id {
                return Some(node);
            } else if node_id > id {
                return None;
            }
        }
        None
    }

    fn valid_offset(&self, offset: u32) -> bool {
        let bits = offset - self.heap_base;
        let mask = (1 << BASE_ORDER) - 1;
        bits & mask == 0
    }

    fn get_node_id_by_offset(&self, offset: u32) -> u32 {
        (offset - self.heap_base) >> BASE_ORDER
    }

    fn get_offset_by_node_id(&self, id: u32) -> u32 {
        (id << BASE_ORDER) + self.heap_base
    }

    fn size_to_order(&self, size: u32) -> u8 {
        self.max_order - max(smallest_order(size) as i8 - BASE_ORDER as i8, 0) as u8
    }
}

fn smallest_order(n: u32) -> u8 {
    (n as f64).log2().ceil() as u8
}

fn largest_order(n: u32) -> u8 {
    (n as f64).log2().floor() as u8
}

#[derive(Clone, PartialEq)]
struct NodeRef(Rc<RefCell<Node>>);

struct Node {
    pub next: Option<NodeRef>,
    pub prev: Option<NodeRef>,
    pub buddies: HashMap<u8, NodeRef>,
    pub free: bool,
    pub order: u8,
    pub max_order: u8,
    pub id: u32,
}

struct NodeIter(Option<NodeRef>);

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref {{ {:?} }}", self.0.borrow())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node {{ prev: {:?}, id: {}, order: {}, free: {}, next: {:?} }}",
            self.prev.clone().map(|n| n.get_id()),
            self.id,
            self.order,
            self.free,
            self.next.clone().map(|n| n.get_id())
        )
    }
}

impl Iterator for NodeIter {
    type Item = NodeRef;

    fn next(&mut self) -> Option<NodeRef> {
        let current = self.0.clone();
        if let Some(node) = current {
            self.0 = node.next();
            Some(node)
        } else {
            None
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
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

    pub fn iter(&self) -> NodeIter {
        NodeIter(Some(self.clone()))
    }

    fn set_next(&self, next: Option<NodeRef>) {
        self.0.borrow_mut().next = next
    }

    fn set_prev(&self, prev: Option<NodeRef>) {
        self.0.borrow_mut().prev = prev
    }

    pub fn get_buddy(&self) -> Option<NodeRef> {
        self.0
            .borrow()
            .buddies
            .get(&self.get_order())
            .map(Clone::clone)
    }

    fn insert_buddy(&self, order: u8, buddy: NodeRef) {
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

    fn max_order(&self) -> u8 {
        self.0.borrow().max_order
    }

    pub fn get_order(&self) -> u8 {
        self.0.borrow().order
    }

    fn set_order(&self, order: u8) {
        self.0.borrow_mut().order = order
    }

    fn set_id(&self, id: u32) {
        self.0.borrow_mut().id = id
    }

    pub fn get_id(&self) -> u32 {
        self.0.borrow().id
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
    use super::*;

    // fn tree_string(node: &NodeRef) -> String {
    //     let mut result = String::from("");
    //     let mut node = node.clone();
    //     while node.prev() != None {
    //         node = node.prev().unwrap();
    //     }
    //     for n in node.iter() {
    //         result = format!("{} -> {:?}", result, n);
    //     }
    //     result
    // }

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
    }

    #[test]
    fn allocate() {
        let mut allocator = BuddyAllocator::new(1000, 10000).unwrap();
        assert_eq!(allocator.allocate(100).unwrap(), 1000);
        assert_eq!(allocator.allocate(300).unwrap(), 1512);
        assert_eq!(allocator.allocate(300).unwrap(), 2024);
        assert_eq!(allocator.allocate(12).unwrap(), 1128);
        assert_eq!(allocator.allocate(1024).unwrap(), 3048);
    }

    #[test]
    fn free() {
        let mut allocator = BuddyAllocator::new(1000, 10000).unwrap();
        let _ = allocator.allocate(100).unwrap();
        let ptr = allocator.allocate(300).unwrap();
        let _ = allocator.allocate(300).unwrap();
        let _ = allocator.allocate(12).unwrap();
        let _ = allocator.allocate(1024).unwrap();
        let _ = allocator.free(ptr).unwrap();
        assert_eq!(allocator.allocate(300).unwrap(), ptr);
        assert_eq!(allocator.allocate(300).unwrap(), 2536);
        assert_eq!(allocator.allocate(300).unwrap(), 4072);
    }

    #[test]
    fn reset() {
        let mut allocator = BuddyAllocator::new(1000, 10000).unwrap();
        let _ = allocator.allocate(100).unwrap();
        let _ = allocator.allocate(300).unwrap();
        let _ = allocator.reset().unwrap();
        assert_eq!(allocator.allocate(100).unwrap(), 1000);
    }
}
