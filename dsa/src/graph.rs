use core::fmt;
use std::collections::{HashMap, HashSet};

use std::collections::hash_map::Entry::Vacant;
use std::hash::Hash;


#[derive(Debug, Clone)]
pub struct NodeNotInGraph;



impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub trait Graph<'a, T> where T: 'a + Eq + Hash {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a T, Vec<(&'a T, i32)>>;
    fn adjacency_table(&self) -> &HashMap<&'a T, Vec<(&'a T, i32)>>;

    fn add_node(&mut self, node:&'a T)-> bool {
        if let Vacant(entry) = self.adjacency_table_mutable().entry(node) {
            entry.insert(Vec::new());
            true
        } else {
            false
        }
    }

    fn add_edge(&mut self, edge:(&'a T, &'a T, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);
        self.adjacency_table_mutable().entry(edge.0).and_modify(|e| {
            e.push((edge.1, edge.2));
        });
    }

    fn neighbours(&self, node: &'a T) -> Result<&[(&'a T, i32)], NodeNotInGraph> {
        match self.adjacency_table().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i)
        }
    }

    fn contains(&self, node: &'a T) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&'a T> {
        self.adjacency_table().keys().copied().collect()
    }

    fn edges(&self) -> Vec<(&'a T, &'a T, i32)> {
        self.adjacency_table().iter().flat_map(|(from_node, from_node_neighbours)| {
            from_node_neighbours.iter().map(move |(to_node, weight)| (*from_node, *to_node, *weight))
        }).collect()
    }
}


pub struct DirectedGraph<'a, T> {
    adjacency_table: HashMap<&'a T, Vec<(&'a T, i32)>>,
}


impl<'a, T> Graph<'a,T> for DirectedGraph<'a, T> where T: 'a + Eq + Hash {
    fn new() -> DirectedGraph<'a, T> {
        DirectedGraph {
            adjacency_table: HashMap::new()
        }
    }

    fn adjacency_table(&self) -> &HashMap<&'a T, Vec<(&'a T, i32)>> {
        &self.adjacency_table
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a T, Vec<(&'a T, i32)>> {
        &mut self.adjacency_table
    }
}

pub struct UndirectedGraph<'a, T> {
    adjacency_table: HashMap<&'a T, Vec<(&'a T, i32)>>
}

impl<'a, T> Graph<'a,T> for UndirectedGraph<'a, T> where T: 'a + Eq + Hash {
    fn new() -> UndirectedGraph<'a, T> {
        UndirectedGraph {
            adjacency_table: HashMap::new()
        }
    }

    fn adjacency_table(&self) -> &HashMap<&'a T, Vec<(&'a T, i32)>> {
        &self.adjacency_table
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a T, Vec<(&'a T, i32)>> {
        &mut self.adjacency_table
    }

    fn add_edge(&mut self, edge:(&'a T, &'a T, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);
        self.adjacency_table_mutable().entry(edge.0).and_modify(|e| {
            e.push((edge.1, edge.2));
        });
        self.adjacency_table_mutable().entry(edge.0).and_modify(|e| {
            e.push((edge.0, edge.2));
        });
    }
}