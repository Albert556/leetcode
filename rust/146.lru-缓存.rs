/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */

// @lc code=start
struct LRUCache {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {}

    fn get(&self, key: i32) -> i32 {}

    fn put(&self, key: i32, value: i32) {}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end
fn main() {
    let obj = LRUCache::new(capacity);
    let ret_1: i32 = obj.get(key);
    obj.put(key, value);
}
