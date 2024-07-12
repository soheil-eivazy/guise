// pub struct DepthFirst {
//     graph: Graph,
//     first: Node,
//     last: Node,
//     current: VecDeque<Path>,
//     beaten_path: HashMap<Node, Node>,
//     count: u32,
// }

// impl DepthFirst {
//     pub fn new(graph:Graph) -> Self {
//         let (first, last) = graph.first_and_last();
//         let mut current = VecDeque::new();
//         current.push_back(Path{node:first, parent:None});
//         dbg!(graph.edges.len());
//         DepthFirst { 
//             graph,
//             first,
//             last,
//             current,
//             beaten_path: HashMap::new(),
//             count: 0,
//         }
//     }
// }


// impl GraphSearch for DepthFirst {
//     fn next(&mut self) -> SearchResult {
//         // procedure DFS_iterative(G, v) is
//         //     let S be a stack
//         //     label v as discovered
//         //     S.push(iterator of G.adjacentEdges(v))
//         //     while S is not empty do
//         //         if S.peek().hasNext() then
//         //             w = S.peek().next()
//         //             if w is not labeled as discovered then
//         //                 label w as discovered
//         //                 S.push(iterator of G.adjacentEdges(w))
//         //         else
//         //             S.pop()

//         SearchResult::Failed
//     }
// }