use std::{hash::{BuildHasher, DefaultHasher, Hash, Hasher, RandomState}, sync::atomic::Ordering};

use crossbeam::epoch::{Atomic, Guard, Owned, Shared};
use node::{BinEntry, Node};

/// The largest possible table capacity. This value must be
/// exactly 1<<30 to stay within Java array allocation and indexing
/// bounds for power of two table sizes, and is further required
/// because the top two bits of 32bit hash fields are used for
/// control purposes
const MAXIMUM_CAPACITY: usize = 1<<30;

/// The default initial table capacity. Must be a power of 2
/// (i.e., at least 1) and at most `MAXIMUM_CAPACITY`
const DEFAULT_CAPACITY: usize = 16;

/// The load factor for this table. Overrides of this value in
/// constructors affect only the initial table capacity. The 
/// actual floating point value isn't normally used -- it is
/// simpler to use expressions such as `n - (n>>>2)` for
/// the associated resizing threshold.
const LOAD_FUNCTION: f64 = 0.75;

/// Minimum number of rebinnings per transfer step, Ranges are
/// subdivided to allow multiple resizer threads. This value
/// serves as a lower bound to avoid resizers encountering 
/// excessive memory contention. THe value should be atleast
/// `DEFAULT_CAPACITY`.
const MIN_TRANSFER_STRIDE: usize = 16;

/// The number of bits used for generation stamp in `size_ctl`,
/// Must be at least 6 for 32bit arrays.
const RESIZE_STAMP_BITS: usize = 16;

/// The maximum number of threads that can help resize.
/// Must fit in `32 - RESIZE_STAMP_BITS` bits.
const MAX_RESIZERS: usize = (1 << (32 - RESIZE_STAMP_BITS)) - 1;

/// The bit shift for recording size stamp in `size_ctl`
const RESIZE_STAMP_SHIFT: usize = 32 - RESIZE_STAMP_BITS;

mod node;

pub struct SHashMap<K,V, S = RandomState> {
   table: Atomic<Table<K,V,S>>,
   build_hasher: S
}

impl<K,V,S> SHashMap<K, V, S> where S: BuildHasher, K: Hash {

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }
    pub fn get<'g>(&'g self, key: &K, guard: &'g Guard) -> Option<Shared<'g, V>> {
        let hasher = self.hash(key);
        let table = self.table.load(std::sync::atomic::Ordering::SeqCst, guard);
        if table.is_null() {
            return None;
        }
        if table.bins.len() {
            return None;
        }

        let bini = table.bini(hasher);
        let bin = table.bin(bini, guard);
        if bin.is_null() {
            return None;
        }
        let node = bin.find(hasher, key);
        if node.is_null() {
            return None;
        }
        let v = node.value.load(Ordering::SeqCst, guard);
        assert!(!v.is_null());
        Some(v)
    }


    pub fn insert(&self, key: K, value: V) -> Option<()> {
        todo!()
    }

    pub fn put(&self, key: K, value: V, no_replacement: bool) -> Option<()> {
        let h = self.hash(&key);
        let mut bin_count = 0;
        let guard = crossbeam::epoch::pin();
        let mut table = self.table.load(Ordering::SeqCst, _&guard);
        let mut node = Owned::new(BinEntry::Node(Node {
            key,
            value: Owned::new(value),
            hash:h,
            next: Atomic::null()
        }));
        let old_value = loop {   
            if table.is_null() || table.bins.len() == 0 {
                let table = self.init_table(guard);
                continue;
            }
            let bini = table.bini(h);
            let mut bin = table.bin(bini, guard);
            while bin.is_null() {
                // fast path -> bin is empty so stock us at the front
                match table.cas_bin(bini, bin, node, guard) {
                    Ok(_old_null_ptr) => {
                        self.add_count(1,0);
                        return None;}
                    Err(changed) => {
                        assert!(!changed.current.is_null());
                        node = changed.new;
                        bin = changed.current;
                    }
                }
            }

            match *bin {
                BinEntry::Moved(next_table) => {
                    table = table.help_transfer(next_table);
                    unimplemented!()
                }
                BinEntry::Node(ref head) if no_replacement && head.hash == h && &head.key == &node.key => {
                    // fast path if replacement is disallowed and first bin matches
                    return Some(())
                }
                BinEntry::Node(ref head) => {
                    // bin is non-empty need to link into it, so we must take the lock
                    let _guard = head.lock.lock();
                    // need to check taht this is _still_ the head
                    let current_head = table.bin(bini,guard);
                    if current_head.as_raw() != bin.as_raw() {
                        /// nope --> try again from the start
                        continue;
                    }

                    // yes, it is still the head, we can now "own" the bin
                    // note that there can still be readers in the bin
                    // no other writters.

                    // TODO: TreeBin && ReservationNode
                    let mut bin_count = 1;
                    let mut n = head;
                    let old_val = loop {
                        if n.hash == node.hash && n.key == node.key {
                            // The key already exist in the map!
                            if no_replacement {
                                // the key is not absent, so dont update
                            } else {
                                let now_garbage = n.value.swap(node.value, Ordering::SeqCst, &guard);
                                unimplemented!("need to dispose of garbage");

                            }
                            break Some(())
                        }
                        // TODO: This Ordering can probably be relaxed due to the mutex
                        let next = n.next.load(Ordering::SeqCst, &guard);

                        if next.is_null() {
                            // we're at the end of the bin -- stick the node here!!
                            n.next.store(node, Ordering::SeqCst);
                            break None;
                        }
                        n = next;
                        bin_count+=1;
                    };

                    // TODO: TREEIFY_THRESHILD
                    if old_val.is_none() {
                        // Increment counter
                        self.add_count(1, bin_count);
                    }
                    break old_val;
                }
            }
        }
        todo!()
    }
}

struct Table<K,V,S> {
    bins: [Atomic<node::BinEntry<K,V>>]
}

impl<K,V,S> Table<K,V,S> {

    #[inline]
    fn bini(&self, hash: u64) -> usize {
        let mask = self.bins.len() as u64 - 1;
        (hash & mask) as usize
    }

    #[inline]
    fn bin<'g>(&self, i:usize, guard: &'g Guard) -> Shared<'g,node::BinEntry<K,V>> {
        self.bins[i].load(std::sync::atomic::Ordering::Acquire, guard)
    }

    #[inline]
    fn cas_bin<'g>(&self, i: usize, current: Shared<node::BinEntry<K,V>>, new: Owned<node::BinEntry<K,V>>, guard: &'g Guard) -> Result<Shared<'g, node::BinEntry<K,V>>, crossbeam::epoch::CompareAndSetError<'g, node::BinEntry<K,V>, Owned<node::BinEntry<K,V>>>> {
        self.bins[i].compare_and_set(current, new, Ordering::AcqRel, guard)
    }
}