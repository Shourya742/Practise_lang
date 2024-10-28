use crossbeam::epoch::Atomic;
use std::cell::UnsafeCell;


/// Entry in Bin
/// 
/// will _generally_ be `Node`. Any entry that is not first in the bin, will be a `Node`
pub(crate) enum BinEntry<K,V> {
    Node(Node<K,V>),
}


impl<K,V> BinEntry<K,V>
where K:Eq
{
    pub(crate) fn find(&self, hash: u64, key: &K)-> Option<&Node<K,V>> {
        todo!()
    }
}


pub(crate) struct Node<K,V> {
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<BinEntry<K,V>>
}