use crossbeam::epoch::{Atomic, Guard, Shared};
use parking_lot::{Mutex, RawMutex};
use std::cell::UnsafeCell;


/// Entry in Bin
/// 
/// will _generally_ be `Node`. Any entry that is not first in the bin, will be a `Node`
pub(crate) enum BinEntry<K,V> {
    Node(Node<K,V>),
    Moved(*const super::Table<K,V>)
}


impl<K,V> BinEntry<K,V>
where K:Eq
{
    pub(crate) fn find<'g>(&'g self, hash: u64, key: &K, guard: &'g Guard)-> Shared<'g, Node<K,V>> {
        match *self {
            BinEntry::Node(ref n) => {
                return n.find(hash,key,guard);
            }
            BinEntry::Moved(_) => todo!()
        }
    }
}


pub(crate) struct Node<K,V> {
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: Atomic<V>,
    pub(crate) next: Atomic<Node<K,V>>,
    pub(crate) lock: Mutex<()>
}

impl<K,V> Node<K,V> where K: Eq{
    pub(crate) fn find<'g>(&'g self, hash: u64, key: &K, guard: &'g Guard) -> Shared<'g, Node<K,V>> {
        if self.hash == hash && &self.key == key {
            return Shared::from(self as *const _);
        }
        let next = self.next.load(std::sync::atomic::Ordering::SeqCst, guard);
        if next.is_null() {
            return Shared::null();
        }
        return next.find(hash,key,guard);
    }
}