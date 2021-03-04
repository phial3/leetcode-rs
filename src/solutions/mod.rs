//! Solutions in LeetCode
//! 

use super::list_node::ListNode;
use super::tree_node::TreeNode;
use std::{
    rc::Rc,
    cell::RefCell
};

pub struct Solutions;

impl Solutions {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut prev = None;
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    // pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut head = head;
    //     let mut fast = &head;
    //     let mut slow = &head;
    //     loop {
    //         // fast pointer move 2 steps
    //         if let Some(node) = &fast {
    //             fast = &node.next;
    //         } else {
    //             break;
    //         }
    //         if let Some(node) = &fast {
    //             fast = &node.next;
    //         } else {
    //             break;
    //         }
    //         // slow pointer move 1 step
    //         if let Some(node) = &slow {
    //             slow = &node.next;
    //         } else {
    //             break;
    //         } 
    //     }
    //     let mid_addr = if let Some(node) = slow {
    //         node.as_ref() as *const ListNode
    //     } else {
    //         return None;
    //     };
    //     while let Some(node) = head.as_ref() {
    //         let addr = node.as_ref() as *const ListNode;
    //         if addr != mid_addr {
    //             head = head.unwrap().next;
    //         } else {
    //             break;
    //         }
    //     }
    //     head
    // }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head.clone();
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.unwrap().next;
            fast = fast.unwrap().next.unwrap().next;
        }
        slow
    }

    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res_head = Box::new(ListNode::new(0));
        let mut current = res_head.as_mut();
        let mut l1_cur = l1;
        let mut l2_cur = l2;
        while l1_cur.is_some() && l2_cur.is_some() {
            if let (Some(mut l1_head), Some(mut l2_head)) = (l1_cur.take(), l2_cur.take()) {
                if l1_head.val < l2_head.val {
                    l1_cur = l1_head.next.take();
                    l2_cur = Some(l2_head);
                    current = current.next.get_or_insert(l1_head);
                } else {
                    l2_cur = l2_head.next.take();
                    l1_cur = Some(l1_head);
                    current = current.next.get_or_insert(l2_head);
                }
            }
        }
        if l1_cur.is_some() {
            current.next = l1_cur;
        }
        if l2_cur.is_some() {
            current.next = l2_cur;
        }
        res_head.next
    }

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut res_head = Box::new(ListNode::new(0));
        let mut pos = &mut res_head;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val != val {
                pos = pos.next.get_or_insert(node);
            }
        }
        res_head.next
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut res_head = Box::new(ListNode::new(0));
        let mut prev: Option<&mut Box<ListNode>> = None;
        let mut curr = &mut res_head;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if let Some(prev_node) = prev {
                if node.val != prev_node.val {
                    curr = curr.next.get_or_insert(node);
                }
            } else {
                curr = curr.next.get_or_insert(node);
            }
            prev = Some(curr);
        }
        res_head.next
    }

    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut res_head = Box::new(ListNode::new(0));
        let mut pos = &mut res_head;
        while head.is_some() && head.as_mut().unwrap().next.is_some() {
            let mut prev = head.take().unwrap();
            head = prev.next.take();
            let mut after = head.take().unwrap();
            head = after.next.take();
            pos = pos.next.get_or_insert(after);
            pos = pos.next.get_or_insert(prev);
        }
        if let Some(node) = head.take() {
            pos.next.get_or_insert(node);
        }
        res_head.next
    }

    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut head_pos = head.as_ref();
        let mut res_head = Box::new(ListNode::new(0));
        let mut pos = &mut res_head;
        let mut len = 0;
        while let Some(node) = head_pos {
            head_pos = node.next.as_ref();
            len = len + 1;
        }
        let i = k % len;
        for _ in 0..(len - i) {
            if let Some(mut node) = head.take() {
                head = node.next.take();
                pos = pos.next.get_or_insert(node);
            } else {
                panic!()
            }
        }
        pos = &mut res_head;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            let temp = pos.next.take();
            pos = pos.next.get_or_insert(node);
            if let Some(node) = temp {
                pos.next.get_or_insert(node);
            }
        }
        res_head.next
    }

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p = p_node.borrow();
                let q = q_node.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            },
            _ => false
        }   
    }

    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::inver(&mut root);
        root
    }

    pub fn inver(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            Self::inver(&mut node.left);
            Self::inver(&mut node.right);
            
            let left = std::mem::replace(&mut node.left, None);
            let right = std::mem::replace(&mut node.right, left);
            let _ = std::mem::replace(&mut node.left, right);
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursive(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(left), Some(right)) => {
                    let left = left.borrow();
                    let right = right.borrow();
                    left.val == right.val
                        && recursive(left.left.as_ref(), right.right.as_ref())
                        && recursive(left.right.as_ref(), right.left.as_ref())
                },
                _ => false
            }
        }
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                recursive(node.left.as_ref(), node.right.as_ref())
            }
        }
    }

    pub fn merge_trees(mut root1: Option<Rc<RefCell<TreeNode>>>, mut root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursive(p: &mut Option<Rc<RefCell<TreeNode>>>, q: &mut Option<Rc<RefCell<TreeNode>>>) {
            match (&p, &q) {
                (Some(left), Some(right)) => {
                    let mut left = left.borrow_mut();
                    let mut right = right.borrow_mut();
                    left.val += right.val;
                    recursive(&mut left.left, &mut right.left);
                    recursive(&mut left.right, &mut right.right);
                },
                (None, Some(_)) => {
                     *p = q.take();
                },
                _ => {}
            }
        }
        recursive(&mut root1, &mut root2);
        root1
    }

    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursive(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = &node {
                if n.borrow().val == val {
                    return node.clone();
                }
                if n.borrow().val < val {
                    return recursive(&mut n.borrow_mut().right, val);
                } else {
                    return recursive(&mut n.borrow_mut().left, val);
                }
            } else {
                None
            }
        }
        recursive(&root, val)
    }

    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn need_remove(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match root {
                None => true,
                Some(node) => {
                    if node.borrow().val == 1 {
                        return false;
                    } else {
                        return need_remove(node.borrow().left.as_ref())
                            && need_remove(node.borrow().right.as_ref());
                    }
                }
            }
        }
        fn new_tree(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if !need_remove(root) {
                if let Some(node) = root.clone() {
                    let new_root = Rc::new(RefCell::new(TreeNode::new(node.borrow().val)));
                    new_root.borrow_mut().left = new_tree(node.borrow().left.as_ref());
                    new_root.borrow_mut().right = new_tree(node.borrow().right.as_ref());
                    Some(new_root)
                } else {
                    None
                }
            } else {
                None
            }
        }
        new_tree(root.as_ref())
    }

    pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursive(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                recursive(&mut node.right, sum);
                node.val += *sum;
                *sum = node.val;
                recursive(&mut node.left, sum);
            }
        }
        let mut sum = 0;
        recursive(&mut root, &mut sum);
        root
    }

    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}