use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note that we flip the order here to get the smallest cost first in the `BinaryHeap`.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    // pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
    //     let mut distances = HashMap::new();
    //     distances.insert(from_node_id, 0);

    //     for _ in 0..self.nodes.len() {
    //         for node_id in self.nodes.keys() {
    //             if let Some(edges) = self.edges.get(node_id) {
    //                 for edge in edges {
    //                     let new_distance = distances
    //                         .get(node_id)
    //                         .and_then(|d: &i32| d.checked_add(edge.weight))
    //                         .unwrap_or(i32::MAX);
    //                     let current_distance = distances.get(&edge.node_b_id).unwrap_or(&i32::MAX);
    //                     if new_distance < *current_distance {
    //                         distances.insert(edge.node_b_id, new_distance);
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     distances.get(&to_node_id).cloned().unwrap_or(i32::MAX)
    // }

    pub fn kkomatsu_dijkstra(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        // If from_node_id or to_node_id does not exist, return i32::MAX.
        if !self.nodes.contains_key(&from_node_id) || !self.nodes.contains_key(&to_node_id) {
            return i32::MAX;
        }

        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();

        // The distance to the start node is 0
        distances.insert(from_node_id, 0);
        heap.push(State { cost: 0, position: from_node_id });

        // While there are nodes to process
        while let Some(State { cost, position }) = heap.pop() {
            // Skip this node if we've found a better way
            if let Some(&current_cost) = distances.get(&position) {
                if cost > current_cost {
                    continue;
                }
            }

            // Next, we check each edge from the current node
            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next_cost = cost + edge.weight;
                    let next_position = edge.node_b_id;

                    // If we found a shorter path to the neighbor, then we continue
                    if next_cost < *distances.get(&next_position).unwrap_or(&i32::MAX) {
                        heap.push(State { cost: next_cost, position: next_position });
                        distances.insert(next_position, next_cost);
                    }
                }
            }
        }

        distances.get(&to_node_id).cloned().unwrap_or(i32::MAX)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        let mut graph = Graph::new();
        graph.add_node(Node { id: 1, x: 0, y: 0 });

        assert_eq!(graph.kkomatsu_dijkstra(1, 1), 0);
        assert_eq!(graph.kkomatsu_dijkstra(1, 2), i32::MAX);
    }

    #[test]
    fn test_two_connected_nodes() {
        let mut graph = Graph::new();
        graph.add_node(Node { id: 1, x: 0, y: 0 });
        graph.add_node(Node { id: 2, x: 1, y: 1 });

        graph.add_edge(Edge {
            node_a_id: 1,
            node_b_id: 2,
            weight: 10,
        });

        assert_eq!(graph.kkomatsu_dijkstra(1, 2), 10);
        assert_eq!(graph.kkomatsu_dijkstra(2, 1), 10);
    }

    #[test]
    fn test_multiple_nodes() {
        let mut graph = Graph::new();
        graph.add_node(Node { id: 1, x: 0, y: 0 });
        graph.add_node(Node { id: 2, x: 1, y: 1 });
        graph.add_node(Node { id: 3, x: 2, y: 2 });
        graph.add_node(Node { id: 4, x: 3, y: 3 });

        graph.add_edge(Edge {
            node_a_id: 1,
            node_b_id: 2,
            weight: 5,
        });
        graph.add_edge(Edge {
            node_a_id: 2,
            node_b_id: 3,
            weight: 10,
        });
        graph.add_edge(Edge {
            node_a_id: 3,
            node_b_id: 4,
            weight: 3,
        });
        graph.add_edge(Edge {
            node_a_id: 1,
            node_b_id: 4,
            weight: 20,
        });

        assert_eq!(graph.kkomatsu_dijkstra(1, 4), 18);
        assert_eq!(graph.kkomatsu_dijkstra(1, 3), 15);
        assert_eq!(graph.kkomatsu_dijkstra(2, 4), 13);
    }
}