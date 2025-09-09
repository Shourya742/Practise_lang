use std::{cmp, path::Path};

use crate::{error::Error, node::Node, node_type::{Key, KeyValuePair, NodeType, Offset}, page::Page, pager::Pager, wal::Wal};

pub const MAX_BRANCHING_FACTOR:usize = 200;
pub const NODE_KEYS_LIMIT: usize = MAX_BRANCHING_FACTOR - 1;

/// BTree struct represents an on-disk B+tree.
/// Each node is persisted in the table file, the leaf nodes contains the values.
pub struct BTree {
    pager: Pager,
    b: usize,
    wal: Wal
}

/// BTreeeBuilder is a Builder for the BTree struct.
pub struct BTreeBuilder {
    /// Path to the tree file.
    path: &'static Path,
    /// The BTree parameter, an inner node contains no more than 2*b-1 keys and no less than b-1 keys
    /// and no more than 2*b children and no less than b children.
    b: usize
}


impl BTreeBuilder {
    pub fn new() -> BTreeBuilder {
        BTreeBuilder { path: Path::new(""), b: 0 }
    }

    pub fn path(mut self, path: &'static Path) -> BTreeBuilder {
        self.path = path;
        self
    }

    pub fn b_parameter(mut self, b: usize) -> BTreeBuilder {
        self.b = b;
        self
    }

    pub fn build(&self) -> Result<BTree, Error> {
        if self.path.to_string_lossy() == "" {
            return Err(Error::UnexpectedError);
        }
        if self.b == 0 {
            return Err(Error::UnexpectedError);
        }

        let mut pager = Pager::new(self.path)?;
        let root = Node::new(NodeType::Leaf(vec![]), true, None);
        let root_offset = pager.write_page(Page::try_from(&root)?)?;
        let parent_directory = self.path.parent().unwrap_or_else(|| Path::new("/tmp"));
        let mut wal = Wal::new(parent_directory.to_path_buf())?;
        wal.set_root(root_offset)?;
        Ok(BTree { pager, b: self.b, wal })
    }
}


impl Default for BTreeBuilder {
    fn default() -> Self {
        BTreeBuilder::new().b_parameter(200).path(Path::new("/tmp/db"))
    }
}

impl BTree {
    fn is_node_full(&self, node: &Node) -> Result<bool, Error> {
        match &node.node_type {
            NodeType::Internal(_, keys) => Ok(keys.len() == (2* self.b -1)),
            NodeType::Leaf(pairs) => Ok(pairs.len() == (2*self.b)),
            NodeType::Unexpected => Err(Error::UnexpectedError)
        }
    }

    fn is_node_underflow(&self, node: &Node) -> Result<bool, Error> {
        match &node.node_type {
            // A root cannot really be "underflowing" as it can contain less than b-1 keys / pointers.
            NodeType::Leaf(pairs) => Ok(pairs.len() < (self.b - 1) && !node.is_root),
            NodeType::Internal(_, keys) => Ok(keys.len() < (self.b - 1) && !node.is_root),
            NodeType::Unexpected => Err(Error::UnexpectedError)
        }
    }

    pub fn insert(&mut self, kv: KeyValuePair) -> Result<(), Error> {
        let root_offset = self.wal.get_root()?;
        let root_page = self.pager.get_page(&root_offset)?;
        let new_root_offset: Offset;
        let mut new_root: Node;
        let mut root = Node::try_from(root_page)?;
        if self.is_node_full(&root)? {
            new_root = Node::new(NodeType::Internal(vec![], vec![]), true, None);
            new_root_offset = self.pager.write_page(Page::try_from(&new_root)?)?;
            root.parent_offset = Some(new_root_offset.clone());
            root.is_root = false;
            let (median, sibling) = root.split(self.b)?;
            let old_root_offset = self.pager.write_page(Page::try_from(&root)?)?;
            let sibling_offset = self.pager.write_page(Page::try_from(&sibling)?)?;
            new_root.node_type = NodeType::Internal(vec![old_root_offset, sibling_offset], vec![median]);
            self.pager.write_page_at_offset(Page::try_from(&new_root)?, &new_root_offset)?;
        } else {
            new_root = root.clone();
            new_root_offset = self.pager.write_page(Page::try_from(&new_root)?)?;
        }
        self.insert_non_full(&mut new_root, new_root_offset.clone(), kv)?;
        self.wal.set_root(new_root_offset)
    }

    fn insert_non_full(
        &mut self,
        node: &mut Node,
        node_offset: Offset,
        kv: KeyValuePair,
    ) -> Result<(), Error> {
        match &mut node.node_type {
            NodeType::Leaf(pairs) => {
                let idx = pairs.binary_search(&kv).unwrap_or_else(|x| x);
                pairs.insert(idx, kv);
                self.pager
                    .write_page_at_offset(Page::try_from(&*node)?, &node_offset)
            }
            NodeType::Internal(children, keys) => {
                let idx = keys
                    .binary_search(&Key(kv.key.clone()))
                    .unwrap_or_else(|x| x);
                let child_offset = children.get(idx).ok_or(Error::UnexpectedError)?.clone();
                let child_page = self.pager.get_page(&child_offset)?;
                let mut child = Node::try_from(child_page)?;
                // Copy each branching-node on the root-to-leaf walk.
                // write_page appends the given page to the db file thus creating a new node.
                let new_child_offset = self.pager.write_page(Page::try_from(&child)?)?;
                // Assign copied child at the proper place.
                children[idx] = new_child_offset.to_owned();
                if self.is_node_full(&child)? {
                    // split will split the child at b leaving the [0, b-1] keys
                    // while moving the set of [b, 2b-1] keys to the sibling.
                    let (median, mut sibling) = child.split(self.b)?;
                    self.pager
                        .write_page_at_offset(Page::try_from(&child)?, &new_child_offset)?;
                    // Write the newly created sibling to disk.
                    let sibling_offset = self.pager.write_page(Page::try_from(&sibling)?)?;
                    // Siblings keys are larger than the splitted child thus need to be inserted
                    // at the next index.
                    children.insert(idx + 1, sibling_offset.clone());
                    keys.insert(idx, median.clone());

                    // Write the parent page to disk.
                    self.pager
                        .write_page_at_offset(Page::try_from(&*node)?, &node_offset)?;
                    // Continue recursively.
                    if kv.key <= median.0 {
                        self.insert_non_full(&mut child, new_child_offset, kv)
                    } else {
                        self.insert_non_full(&mut sibling, sibling_offset, kv)
                    }
                } else {
                    self.pager
                        .write_page_at_offset(Page::try_from(&*node)?, &node_offset)?;
                    self.insert_non_full(&mut child, new_child_offset, kv)
                }
            }
            NodeType::Unexpected => Err(Error::UnexpectedError),
        }
    }

    pub fn search(&mut self, key: String) -> Result<KeyValuePair, Error> {
        let root_offset = self.wal.get_root()?;
        let root_page = self.pager.get_page(&root_offset)?;
        let root = Node::try_from(root_page)?;
        self.search_node(root, &key)
    }

    fn search_node(&mut self, node: Node, search: &str) -> Result<KeyValuePair, Error> {
        match node.node_type {
            NodeType::Internal(children, keys) => {
                let idx = keys.binary_search(&Key(search.to_string())).unwrap_or_else(|x| x);
                let child_offset = children.get(idx).ok_or(Error::UnexpectedError)?;
                let page = self.pager.get_page(child_offset)?;
                let child_node = Node::try_from(page)?;
                self.search_node(child_node, search)
            }
            NodeType::Leaf(pairs) => {
                if let Ok(idx) = pairs.binary_search_by_key(&search.to_string(), |pair| pair.key.clone()) {
                    return Ok(pairs[idx].clone());
                }
                Err(Error::KeyNotFound)
            },
            NodeType::Unexpected => Err(Error::UnexpectedError)
        }
    }

    pub fn delete(&mut self, key: Key) -> Result<(), Error> {
        let root_offset = self.wal.get_root()?;
        let root_page = self.pager.get_page(&root_offset)?;
        let mut new_root = Node::try_from(root_page)?;
        let new_root_page = Page::try_from(&new_root)?;
        let new_root_offset = self.pager.write_page(new_root_page)?;
        self.delete_key_from_subtree(key, &mut new_root, &new_root_offset)?;
        self.wal.set_root(new_root_offset)

    }

    fn delete_key_from_subtree(&mut self, key: Key, node: &mut Node, node_offset: &Offset) -> Result<(), Error> {
        match &mut node.node_type {
            NodeType::Leaf(pairs) => {
                let key_idx = pairs.binary_search_by_key(&key, |kv| Key(kv.key.clone())).map_err(|_| Error::KeyNotFound)?;
                pairs.remove(key_idx);
                self.pager.write_page_at_offset(Page::try_from(&*node)?, node_offset)?;
                self.borrow_if_needed(node.to_owned(), &key)?;
            },
            NodeType::Internal(children, keys) => {
                let node_idx = keys.binary_search(&key).unwrap_or_else(|x| x);

                let child_offset = children.get(node_idx).ok_or(Error::UnexpectedError)?;
                let child_page = self.pager.get_page(child_offset)?;
                let mut child_node = Node::try_from(child_page)?;
                child_node.parent_offset = Some(node_offset.to_owned());
                let new_child_page = Page::try_from(&child_node)?;
                let new_child_offset = self.pager.write_page(new_child_page)?;
                children[node_idx] = new_child_offset.to_owned();
                self.pager.write_page_at_offset(Page::try_from(&*node)?, node_offset)?;
                return self.delete_key_from_subtree(key, &mut child_node, &new_child_offset);
            }
            NodeType::Unexpected => return Err(Error::UnexpectedError)
        }
        Ok(())
    }

    fn borrow_if_needed(&mut self, node: Node, key: &Key) -> Result<(), Error> {
        if self.is_node_underflow(&node)? {
            let parent_offset = node.parent_offset.clone().ok_or(Error::UnexpectedError)?;
            let parent_page = self.pager.get_page(&parent_offset)?;
            let mut parent_node = Node::try_from(parent_page)?;
            match parent_node.node_type {
                NodeType::Internal(ref mut children, ref mut keys ) => {
                    let idx = keys.binary_search(key).unwrap_or_else(|x| x);
                    let sibling_idx;
                    match idx > 0 {
                        false => sibling_idx = idx + 1,
                        true => sibling_idx = idx - 1
                    }

                    let sibling_offset = children.get(sibling_idx).ok_or(Error::UnexpectedError)?;
                    let sibling_page = self.pager.get_page(sibling_offset)?;
                    let sibling = Node::try_from(sibling_page)?;

                    let merged_node = self.merge(node, sibling)?;
                    let merged_node_offset = self.pager.write_page(Page::try_from(&merged_node)?)?;
                    let merged_node_idx = cmp::min(idx, sibling_idx);

                    children.remove(merged_node_idx);
                    children.remove(merged_node_idx);

                    if parent_node.is_root && children.is_empty() {
                        self.wal.set_root(merged_node_offset)?;
                        return Ok(());
                    }

                    keys.remove(idx);
                    children.insert(merged_node_idx, merged_node_offset);
                    self.pager.write_page_at_offset(Page::try_from(&parent_node)?, &parent_offset)?;
                    return self.borrow_if_needed(parent_node, key);
                }
                _ => return Err(Error::UnexpectedError)
            }
        }
        Ok(())
    }


    fn merge(&self, first: Node, second: Node) -> Result<Node, Error> {
        match first.node_type {
            NodeType::Leaf(first_pairs) => {
                if let NodeType::Leaf(second_pairs) = second.node_type {
                    let merged_pairs: Vec<KeyValuePair> = first_pairs
                        .into_iter()
                        .chain(second_pairs.into_iter())
                        .collect();
                    let node_type = NodeType::Leaf(merged_pairs);
                    Ok(Node::new(node_type, first.is_root, first.parent_offset))
                } else {
                    Err(Error::UnexpectedError)
                }
            }
            NodeType::Internal(first_offsets, first_keys) => {
                if let NodeType::Internal(second_offsets, second_keys) = second.node_type {
                    let merged_keys: Vec<Key> = first_keys
                        .into_iter()
                        .chain(second_keys.into_iter())
                        .collect();
                    let merged_offsets: Vec<Offset> = first_offsets
                        .into_iter()
                        .chain(second_offsets.into_iter())
                        .collect();
                    let node_type = NodeType::Internal(merged_offsets, merged_keys);
                    Ok(Node::new(node_type, first.is_root, first.parent_offset))
                } else {
                    Err(Error::UnexpectedError)
                }
            }
            NodeType::Unexpected => Err(Error::UnexpectedError),
        }
    }

    fn print_sub_tree(&mut self, prefix: String, offset: Offset) -> Result<(), Error> {
        println!("{}Node at offset: {}", prefix, offset.0);
        let curr_prefix = format!("{}|->", prefix);
        let page = self.pager.get_page(&offset)?;
        let node = Node::try_from(page)?;
        match node.node_type {
            NodeType::Internal(children, keys) => {
                println!("{}Keys: {:?}", curr_prefix, keys);
                println!("{}Children: {:?}", curr_prefix, children);
                let child_prefix = format!("{}   |  ", prefix);
                for child_offset in children {
                    self.print_sub_tree(child_prefix.clone(), child_offset)?;
                }
                Ok(())
            }
            NodeType::Leaf(pairs) => {
                println!("{}Key value pairs: {:?}", curr_prefix, pairs);
                Ok(())
            }
            NodeType::Unexpected => Err(Error::UnexpectedError),
        }
    }
    
    pub fn print(&mut self) -> Result<(), Error> {
        println!();
        let root_offset = self.wal.get_root()?;
        self.print_sub_tree("".to_string(), root_offset)
    }

}