//! Solutions in LeetCode
//! 

use super::list_node::ListNode;
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
}