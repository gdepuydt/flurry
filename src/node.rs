use std::cell::UnsafeCell;
use crossbeam::epoch::Atomic;

pub(crate) enum BinEntry<K, V> {
    Node(Node<K, V>),
}

impl<K, V> BinEntry<K, V>
where 
    K: Eq, 
{
    pub(crate) fn find(&self, hash: u64, key: &K) -> Option<&Node<K, V>> {
        match *self {
            BinEntry::Node(ref start) => {
                // TODO

                }
            }
        }
    }
}

pub(crate) struct Node<K, V> {
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<BinEntry<K, V>>,
}