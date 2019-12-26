use std::cell::UnsafeCell;
use crossbeam::epoch::{Atomic, Guard, Shared};
use std::sync::atomic::Ordering;

pub(crate) enum BinEntry<K, V> {
    Node(Node<K, V>),
}

impl<K, V> BinEntry<K, V>
where 
    K: Eq, 
{
    pub(crate) fn find<'g>(&'g self, hash: u64, key: &K, guard: &'g Guard) -> Shared<'g, Node<K, V>> {
        match *self {
            BinEntry::Node(ref n) => {
                if n.hash == hash && &n.key == key {
                    return Shared::from(n as *const _);
                }
                let next = n.next.load(Ordering::SeqCst, guard);
                if next.is_null() {
                    return Shared::null();
                }
                return next;
            }
        }
    }
}

pub(crate) struct Node<K, V> {
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<Node<K, V>>,
}