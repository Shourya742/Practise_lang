
/// A single page size.
/// Each page represents a node in the Btree
pub const PAGE_SIZE: usize = 4096;

pub const PTR_SIZE: usize = size_of::<usize>();

/// Common node header layout
pub const IS_ROOT_SIZE: usize = 1;
pub const IS_ROOT_OFFSET: usize = 0;
pub const NODE_TYPE_SIZE: usize = 1;
pub const NODE_TYPE_OFFSET: usize = 1;
pub const PARENT_POINTER_OFFSET: usize = 2;
pub const PARENT_POINTER_SIZE: usize = PTR_SIZE;
pub const COMMON_NODE_HEADER_SIZE: usize = NODE_TYPE_SIZE + IS_ROOT_SIZE + PARENT_POINTER_SIZE;

/// Leaf node header layout (18 bytes in total)
/// 
/// Space for keys and values: PAGE_SIZE - LEAF_NODE_HEADER_SIZE = 4096 - 18 = 4078 bytes.
/// Which leaves 4076 / key_limit = 20 (ten for key and 10 for value).
pub const LEAF_NODE_NUM_PAIRS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const LEAF_NODE_NUM_PAIRS_SIZE: usize = PTR_SIZE;
pub const LEAF_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + LEAF_NODE_NUM_PAIRS_SIZE;

/// Internal header layout (18 bytes in total).
/// 
/// Space for children and keys: PAGE_SIZE - INTERNAL_NODE_HEADER_SIZE = 4096 - 18 = 4078 bytes.
pub const INTERNAL_NODE_NUM_CHILDREN_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const INTERNAL_NODE_NUM_CHILDREN_SIZE: usize = PTR_SIZE;
pub const INTERNAL_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + INTERNAL_NODE_NUM_CHILDREN_SIZE;

/// Key, Value sizes.
pub const KEY_SIZE: usize = 10;
pub const VALUE_SIZE: usize = 10;

/// Wrappers for converting byte to bool and back.
/// The convention used throughout the index file is: one is true; otherwise - false.
pub trait FromByte {
    fn from_byte(&self) -> bool;
}

pub trait ToByte {
    fn to_byte(&self) -> u8;
}

impl FromByte for u8 {
    fn from_byte(&self) -> bool {
        matches!(self, 0x01)
    }
}

impl ToByte for bool {
    fn to_byte(&self) -> u8 {
        match self {
            true => 0x01,
            false => 0x00,
        }
    }
}