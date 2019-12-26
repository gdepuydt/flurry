const MAXIMUM_CAPACITY: usize = 1 << 30;
const DEFAULT_CAPACITY: usize = 16;
const LOAD_FACTOR: f64 = 0.75;
const MIN_TRANSFER_STRIDE: usize = 16;
const RESIZE_STAMP_BITS: usize = 16;
const MAX_RESIZERS: usize = (1 << (32 - RESIZE_STAMP_BITS)) -1;
const RESIZE_STAMP_SHIFT: usize = 32 - RESIZE_STAMP_BITS;

mod node;

use crossbeam::epoch::{Atomic, Shared, Guard};
use std::sync::atomic::Ordering;

pub struct FlurryHashMap<K, V> {
    table: Atomic<Table<K, V>>, 
    
}

struct Table<K, V> {
    bins: [Atomic<node::BinEntry<K, V>>],
}

impl Table<K, V> {
    fn at<'g>(&'g self, i: usize, guard: &'g Guard) -> Shared<'g, node::BinEntry<K, V>> {
        self.bins[i].load(Ordering::Acquire, guard)
    }
}