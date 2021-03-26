//! Solutions in LeetCode
//! 

mod lru;

use super::list_node::ListNode;
use super::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

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

    pub fn delete_duplicates_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut head = head;
        let mut res_head = Box::new(ListNode::new(0));
        let mut prev = None;
        let mut curr = &mut res_head;
        let mut num = 0;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if prev.is_none() {
                prev = Some(node);
            } else {
                if node.val != prev.as_ref().unwrap().val {
                    if num == 0 {
                        let prev_node = prev.take().unwrap();
                        curr = curr.next.get_or_insert(prev_node);
                    }
                    if head.is_none() {
                        curr = curr.next.get_or_insert(node);
                    } else {
                        prev = Some(node);
                        num = 0;
                    }
                } else {
                    num += 1;
                }
            }
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
        fn recursive(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, n: i32) {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                let mut left = node.left.clone();
                let mut right = node.right.clone();
                if depth == n - 1 {
                    node.left = Some(
                        Rc::new(RefCell::new(TreeNode {
                            val,
                            left,
                            right: None,
                        }))
                    );
                    node.right = Some(
                        Rc::new(RefCell::new(TreeNode {
                            val,
                            left: None,
                            right,
                        }))
                    );
                } else {
                    recursive(&mut left, val, depth + 1, n);
                    recursive(&mut right, val, depth + 1, n);
                }
            }
        }
        if d == 1 {
            let res = Some(
                Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: root.clone(),
                    right: None
                }))
            );
            return  res;
        }
        let mut root = root;
        recursive(&mut root, v, 1, d);
        root
    }

    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut res_head = Box::new(ListNode::new(0));
        let mut pos = res_head.as_mut();
        let mut counter = 1;
        let mut nodes = Vec::new();
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if counter < left {
                pos = pos.next.get_or_insert(node);
                counter += 1;
            } else if counter <= right {
                nodes.push(node);
                if counter == right {
                    break;
                } else {
                    counter += 1;
                }
            } else {
                unreachable!()
            }
        }
        while let Some(node) = nodes.pop() {
            pos = pos.next.get_or_insert(node);
        }
        while let Some(mut node) = head.take() {
            head = node.next.take();
            pos = pos.next.get_or_insert(node);
        }
        res_head.next
    }

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursive(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.len() == 0 {
                return None;
            }
            let mut max = nums[0];
            let mut max_pos = 0;
            for i in 0..nums.len() {
                if nums[i] > max {
                    max = nums[i];
                    max_pos = i;
                }
            }
            
            let root = Rc::new(RefCell::new(TreeNode::new(max)));
            root.borrow_mut().left = recursive(nums[0..max_pos].to_vec());
            if max_pos < nums.len() - 1 {
                root.borrow_mut().right = recursive(nums[(max_pos+1)..].to_vec());
            }
            Some(root)
        }

        recursive(nums)
    }

    pub fn int_to_roman(num: i32) -> String {
        // ref: https://leetcode.com/problems/integer-to-roman/discuss/1016135/Rust%3A-vector-solution
        let m = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let n = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

        let (mut num, mut s) = (num, String::new());
        for i in 0..13 {
            let mut j = num / m[i];
            num %= m[i];
            while j > 0 {
                s.push_str(n[i]);
                j -= 1;
            }
        }
        s
    }

    #[allow(non_snake_case)]
    pub fn LRU(&self, operators: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        assert!(k > 0);
        struct Lru {
            inner: Vec<((i32, i32), i32)>,
            max_size: usize
        }
        impl Lru {
            pub fn new(max_size: usize) -> Self {
                Self {
                    inner: Vec::new(),
                    max_size
                }
            }
            pub fn find_retire(&self) -> usize {
                let mut index = 0;
                let mut longest = self.inner[0].1;
                for (i, (_, time)) in self.inner.iter().enumerate() {
                    if *time > longest {
                        longest = *time;
                        index = i;
                    }
                }
                index
            }
            pub fn all_add(&mut self) {
                for (_, time) in &mut self.inner {
                    *time += 1;
                }
            }
            pub fn set(&mut self, val: (i32, i32)) {
                self.all_add();
                if self.inner.len() >= self.max_size {
                    let index = self.find_retire();
                    self.inner[index] = (val, 0);
                } else {
                    self.inner.push((val, 0));
                }
            }
            pub fn get(&mut self, key: i32) -> i32 {
                self.all_add();
                for ((k, v), time) in &mut self.inner {
                    if *k == key {
                        *time = 0;
                        return *v;
                    }
                }
                -1
            }
        }
        let mut res = Vec::new();
        let mut lru = Lru::new(k as usize);
        for ops in operators {
            assert!(ops.len() >= 2);
            match ops[0] {
                1 => {
                    assert_eq!(ops.len(), 3);
                    lru.set((ops[1], ops[2]));
                },
                2 => {
                    assert_eq!(ops.len(), 2);
                    res.push(lru.get(ops[1]));
                },
                _ => panic!("error input!")
            }
        }
        res
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Ref: https://leetcode.com/problems/3sum/discuss/218353/Rust-32ms
        let mut res = Vec::with_capacity(nums.len() / 3);
        if nums.len() < 3 {
            return res;
        }
        nums.sort();
        let mut left;
        let mut right;
        let mut result;

        for (i, num) in nums.iter().enumerate() {
            if *num > 0 {
                break;
            }
            left = i + 1;
            right = nums.len() - 1;
            while left < right {
                result = *num + nums[left] + nums[right];
                if result == 0 {
                    // 检查是否有重复
                    let mut has_duplicates = false;
                    let mut rev = res.len() as i32 - 1;
                    let mut prev;
                    while rev >= 0 {
                        prev = &res[rev as usize];
                        if prev[0] == *num {
                            if prev[1] == nums[left] && prev[2] == nums[right] {
                                has_duplicates = true;
                                break;
                            }
                        } else {
                            // 前面不可能有一样的
                            break;
                        }
                        rev -= 1;
                    }
                    if !has_duplicates {
                        res.push(vec![*num, nums[left], nums[right]]);
                    }
                    left += 1;
                    right -= 1;
                } else if result > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        res
    }

    pub fn longest_palindrome(s: String) -> String {
        let data = s.as_bytes();
        let c = '#' as u8;
        let mut new_s = Vec::new();
        for i in 0..data.len() {
            new_s.push(c);
            new_s.push(data[i]);
        }
        new_s.push(c);
        let mut res = Vec::new();
        for i in 0..new_s.len() {
            let mut left = i as i32 - 1;
            let mut right = i as i32 + 1;
            while left >= 0 && right < new_s.len() as i32{
                if new_s[left as usize] == new_s[right as usize] {
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
            if right - left - 1 > res.len() as i32 {
                res = new_s[(left+1) as usize..(right-1) as usize].to_vec();
            }
        }
        let mut return_s = Vec::new();
        while let Some(item) = res.pop() {
            if item != '#' as u8 {
                return_s.push(item);
            }
        }
        String::from_utf8(return_s).unwrap()
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        // ref: https://leetcode-cn.com/problems/container-with-most-water/solution/rust-on-shuang-zhi-zhen-by-jack_/
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut ans = -1;
        while left < right {
            ans = ans.max(height[left].min(height[right]) * (right as i32 - left as i32));
            if height[left] < height[right] { left += 1;}
            else { right -= 1;}
        }
        ans
    }
    
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 { return s; }
        let mut res_s = String::new();
        let mut res = Vec::new();
        for _ in 0..num_rows {
            res.push(Vec::new());
        }
        let mut pos = 0;
        let mut up = false;
        let mut data = s.as_bytes().to_vec();
        while let Some(_) = data.first() {
            let c = data.remove(0);
            res[pos].push(c);
            if pos == 0 {
                pos += 1;
                up = false;
            } else if pos == num_rows as usize - 1 {
                pos -= 1;
                up = true;
            } else {
                if up {
                    pos -= 1;
                } else {
                    pos += 1;
                }
            }
        }
        for v in res {
            for c in v {
                res_s.push(c as char);
            }
        }
        res_s
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;
        fn backtrack(
            res_vec: &mut Vec<String>,
            map: &HashMap<char, String>,
            digits: &String,
            index: usize,
            res: &mut String
        ) {
            if index == digits.len() {
                res_vec.push(String::from(res.as_str()));
            } else {
                let digit  = digits.as_bytes()[index] as char;
                if let Some(s) = map.get(&digit) {
                    let new_s = String::from(s.as_str());
                    for ch in new_s.chars() {
                        res.push(ch);
                        backtrack(res_vec, map, digits, index + 1, res);
                        res.remove(index);
                    }
                }
            }
        }
        if digits.len() == 0 {
            return Vec::new();
        }
        let mut map: HashMap<char, String> = HashMap::new();
        map.insert('2', "abc".to_string());
        map.insert('3', "def".to_string());
        map.insert('4', "ghi".to_string());
        map.insert('5', "jkl".to_string());
        map.insert('6', "mno".to_string());
        map.insert('7', "pqrs".to_string());
        map.insert('8', "tuv".to_string());
        map.insert('9', "wxyz".to_string());
        let mut res_vec = Vec::new();
        let mut res = String::new();
        backtrack(&mut res_vec, &map, &digits, 0, &mut res);
        res_vec
    }
}