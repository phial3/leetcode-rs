struct LRUCache {
    inner: Vec<((i32, i32), i32)>,
    max_size: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        assert!(capacity > 0);
        Self {
            inner: Vec::new(),
            max_size: capacity as usize
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        self.all_add();
        for ((k, v), time) in &mut self.inner {
            if *k == key {
                *time = 0;
                return *v;
            }
        }
        -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.all_add();
        for ((k, v), time) in &mut self.inner {
            if *k == key {
                *v = value;
                *time = 0;
                return;
            }
        }  
        if self.inner.len() >= self.max_size {
            let index = self.find_retire();
            self.inner[index] = ((key, value), 0);
        } else {
            self.inner.push(((key, value), 0));
        }
    }

    fn find_retire(&self) -> usize {
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
    fn all_add(&mut self) {
        for (_, time) in &mut self.inner {
            *time += 1;
        }
    }
}
