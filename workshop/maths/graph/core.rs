use std::collections::HashMap;

// use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    pub x: u16,
    pub y: u16,
}

impl Node {
    pub fn new(x: u16, y: u16) -> Node {
        Node {x, y}
    }
}


#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> bool {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
            return false
        }

        true
    }

    pub fn add_edge(&mut self, edge: (Node, Node)) {

        self.edges.entry(edge.0)
            .and_modify(|e| e.push(edge.1.clone()))
            .or_insert_with(|| vec![edge.1.clone()]);

        self.edges.entry(edge.1)
            .and_modify(|e| e.push(edge.0.clone()))
            .or_insert_with(|| vec![edge.0]);
    }

    pub fn first_and_last(&self) -> (Node, Node) {
        let mut res = self.nodes.clone();
        res.sort_unstable_by_key(|item| (item.y, item.x));

        (res.first().unwrap().to_owned(), res.last().unwrap().to_owned())
    }


    pub fn load_edges(&self, node: &Node) -> Vec<Node> {
        self.edges.get(node).unwrap_or(&Vec::new()).clone()
    }
}



pub trait GraphSearch {
    fn next(&mut self) -> SearchResult;
}

pub enum SearchResult {
    Done(Vec<Node>),
    Failed,
    Next(HashMap<Node, Vec<Node>>),
}

#[derive(Copy, Clone, Debug, Hash, PartialOrd)]
pub struct Path {
    pub node: Node,
    pub parent: Option<Node>
}

impl Path {
    pub fn new(node: Node, parent: Option<Node>) -> Self {
        Path{node, parent}
    }
}

impl PartialEq for Path {
    fn eq(&self, node: &Path) -> bool {
        self.node == node.node
    }

    fn ne(&self, node: &Path) -> bool {
        self.node != node.node
    }
}




