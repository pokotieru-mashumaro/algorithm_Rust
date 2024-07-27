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

    pub fn bellman_ford(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        distances.insert(from_node_id, 0);

        for _ in 0..self.nodes.len() {
            for node_id in self.nodes.keys() {
                if let Some(edges) = self.edges.get(node_id) {
                    for edge in edges {
                        let new_distance = distances
                            .get(node_id)
                            .and_then(|d: &i32| d.checked_add(edge.weight))
                            .unwrap_or(i32::MAX);
                        let current_distance = distances.get(&edge.node_b_id).unwrap_or(&i32::MAX);
                        if new_distance < *current_distance {
                            distances.insert(edge.node_b_id, new_distance);
                        }
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

        assert_eq!(graph.bellman_ford(1, 1), 0);
        assert_eq!(graph.bellman_ford(1, 2), i32::MAX);
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

        assert_eq!(graph.bellman_ford(1, 2), 10);
        assert_eq!(graph.bellman_ford(2, 1), 10);
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

        assert_eq!(graph.bellman_ford(1, 4), 18);
        assert_eq!(graph.bellman_ford(1, 3), 15);
        assert_eq!(graph.bellman_ford(2, 4), 13);
    }
}