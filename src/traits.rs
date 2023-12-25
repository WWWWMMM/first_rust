pub trait Traversal {
    type Item;
    fn reset(&self, chunk_size : u32);
    // return true 表示还可以继续遍历，线程安全
    fn traversal(&self, process: impl Fn(&[Self::Item]) -> bool);
}