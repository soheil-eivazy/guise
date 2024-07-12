use super::core::{Graph, GraphSearch, Node, Path, SearchResult};
use std::collections::{HashMap, VecDeque};
pub struct BreadthFirst {
    graph: Graph,
    first: Node,
    last: Node,
    current: VecDeque<Path>,
    beaten_path: HashMap<Node, Node>,
    count: u32,
}

impl BreadthFirst {
    pub fn new(graph:Graph) -> Self {
        let (first, last) = graph.first_and_last();
        let mut current = VecDeque::new();
        current.push_back(Path::new(first, None));
        dbg!(graph.edges.len());
        BreadthFirst { 
            graph,
            first,
            last,
            current,
            beaten_path: HashMap::new(),
            count: 0,
        }
    }
}

impl GraphSearch for BreadthFirst {
    fn next(&mut self) -> SearchResult {
        self.count += 1;
        // let start = Instant::now();

        let cur_len = self.current.len();
        let mut result = HashMap::new();

        for _ in 0..cur_len {
            let node =  self.current.pop_front().unwrap();
            if node.node == self.last {
                let mut parent = node.parent;
                let mut has_parent = true;
                let mut solved = Vec::<Node>::new();
                solved.push(node.node);
                while has_parent {
                    solved.push(parent.unwrap());
                    if let Some(p) = self.beaten_path.get(&parent.unwrap()) {
                        parent = Some(*p);
                        if *p == self.first {
                            solved.push(parent.unwrap());
                            has_parent = false;
                        }
                    } else {
                        has_parent = false;
                    }
                }

                return SearchResult::Done(solved);
            }

            let mut n = Vec::new();
            for edge in self.graph.load_edges(&node.node) {
                
                if !self.beaten_path.contains_key(&edge) {
                    n.push(edge);
                    self.current.push_back(Path{node: edge, parent: Some(node.node)});
                    self.beaten_path.insert(edge, node.node);
                } 
            }

            result.insert(node.node, n);
        }

        if self.current.is_empty() {
            return SearchResult::Failed
        }

        // let duration = start.elapsed();
        // dbg!(self.count, cur_len, self.beaten_path.len(), duration);

        SearchResult::Next(result)
    }
}
