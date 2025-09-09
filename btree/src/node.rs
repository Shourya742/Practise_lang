use crate::{error::Error, node_type::{Key, KeyValuePair, NodeType, Offset}, page::Page, page_layout::{FromByte, INTERNAL_NODE_HEADER_SIZE, INTERNAL_NODE_NUM_CHILDREN_OFFSET, IS_ROOT_OFFSET, KEY_SIZE, LEAF_NODE_HEADER_SIZE, LEAF_NODE_NUM_PAIRS_OFFSET, NODE_TYPE_OFFSET, PARENT_POINTER_OFFSET, PTR_SIZE, VALUE_SIZE}};

/// Node represents a node in the BTree occupied by a single page in memory.
#[derive(Clone, Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub is_root: bool,
    pub parent_offset: Option<Offset>
}

impl Node {
    pub fn new(node_type: NodeType, is_root: bool, parent_offset: Option<Offset>) -> Node {
        Node {
            node_type,
            is_root,
            parent_offset
        }
    }

    /// split creates a sibling node from a given node by splitting the node in two around a median.
    /// split will split the child at b leaving the [0, b-1] keys
    /// while moving the set of [b, 2b-1] keys to the sibling.
    pub fn split(&mut self, b: usize) -> Result<(Key, Node), Error> {
        match self.node_type {
            NodeType::Internal(ref mut children, ref mut keys ) => {
                let mut sibling_keys = keys.split_off(b-1);
                let median_key = sibling_keys.remove(0);
                let sibling_children = children.split_off(b);
                Ok((median_key, Node::new(NodeType::Internal(sibling_children, sibling_keys), false, self.parent_offset.clone())))
            }
            NodeType::Leaf(ref mut pairs) => {
                // Populate siblings pairs.
                let sibling_pairs = pairs.split_off(b);
                // Pop median key.
                let median_pair = pairs.get(b - 1).ok_or(Error::UnexpectedError)?.clone();

                Ok((
                    Key(median_pair.key),
                    Node::new(
                        NodeType::Leaf(sibling_pairs),
                        false,
                        self.parent_offset.clone(),
                    ),
                ))
            }
            NodeType::Unexpected => Err(Error::UnexpectedError)
        }
    }
}

impl TryFrom<Page> for Node {
    type Error = Error;
    fn try_from(page: Page) -> Result<Self, Self::Error> {
        let raw = page.get_data();
        let node_type = NodeType::from(raw[NODE_TYPE_OFFSET]);
        let is_root = raw[IS_ROOT_OFFSET].from_byte();
        let parent_offset: Option<Offset>;
        if is_root {
            parent_offset = None;
        } else {
            parent_offset = Some(Offset(page.get_value_from_offset(PARENT_POINTER_OFFSET)?));
        }
        match node_type {
            NodeType::Internal(mut children, mut keys ) => {
                let num_children = page.get_value_from_offset(INTERNAL_NODE_NUM_CHILDREN_OFFSET)?;
                let mut offset = INTERNAL_NODE_HEADER_SIZE;

                for _i in 1..=num_children {
                    let child_offset = page.get_value_from_offset(offset)?;
                    children.push(Offset(child_offset));
                    offset += PTR_SIZE;
                }

                for _i in 1..num_children {
                    let key_raw = page.get_ptr_from_offset(offset, KEY_SIZE);
                    let key = match str::from_utf8(key_raw) {
                        Ok(key) => key,
                        Err(_) => return Err(Error::UTF8Error),
                    };
                    offset += KEY_SIZE;
                    keys.push(Key(key.trim_matches(char::from(0)).to_string()));
                }
                Ok(Node::new(
                    NodeType::Internal(children, keys),
                    is_root,
                    parent_offset,
                ))
            }
            NodeType::Leaf(mut pairs) => {
                let mut offset = LEAF_NODE_NUM_PAIRS_OFFSET;
                let num_keys_val_pairs = page.get_value_from_offset(offset)?;
                offset = LEAF_NODE_HEADER_SIZE;
                for _i in 0..num_keys_val_pairs {
                    let key_raw = page.get_ptr_from_offset(offset, KEY_SIZE);
                    let key = match str::from_utf8(key_raw) {
                        Ok(key) => key,
                        Err(_) => return Err(Error::UTF8Error)
                    };
                    offset += KEY_SIZE;

                    let value_raw = page.get_ptr_from_offset(offset, VALUE_SIZE);
                    let value = match str::from_utf8(value_raw) {
                        Ok(val) => val,
                        Err(_) => return Err(Error::UTF8Error)
                    };
                    offset += VALUE_SIZE;
                    pairs.push(KeyValuePair::new(
                        key.trim_matches(char::from(0)).to_string(),
                        value.trim_matches(char::from(0)).to_string(),
                    ))
                }
                Ok(Node::new(NodeType::Leaf(pairs), is_root, parent_offset))
            }
            NodeType::Unexpected => Err(Error::UnexpectedError),
        }
    }
}