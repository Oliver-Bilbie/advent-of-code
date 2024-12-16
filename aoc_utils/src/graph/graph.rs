use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Edge<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    pub source: NodeId,
    pub destination: NodeId,
    pub weight: i64,
}

#[derive(Clone, Debug)]
pub struct Destination<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    pub node: NodeId,
    pub weight: i64,
}

#[derive(Clone, Debug)]
pub struct Node<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    pub min_distance: Option<u128>,
    pub visited: bool,
    pub destinations: Vec<Destination<NodeId>>,
    pub previous_location: Vec<NodeId>,
}

#[derive(Clone, Debug)]
pub struct Graph<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    nodes: HashMap<NodeId, Node<NodeId>>,
}

impl<NodeId> Graph<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    pub fn new() -> Graph<NodeId> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: NodeId) {
        self.nodes.insert(
            id,
            Node {
                min_distance: None,
                visited: false,
                destinations: vec![],
                previous_location: vec![],
            },
        );
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&Node<NodeId>> {
        self.nodes.get(&id)
    }

    pub fn get_mut_node(&mut self, id: &NodeId) -> Option<&mut Node<NodeId>> {
        self.nodes.get_mut(&id)
    }

    pub fn get_node_distance(&self, id: &NodeId) -> Option<u128> {
        self.get_node(id)?.min_distance
    }

    pub fn add_edge(&mut self, edge: Edge<NodeId>) {
        if !self.nodes.contains_key(&edge.source) {
            self.add_node(edge.source.clone());
        }
        if !self.nodes.contains_key(&edge.destination) {
            self.add_node(edge.destination.clone());
        }

        self.nodes.entry(edge.source).and_modify(|node| {
            node.destinations.push(Destination {
                node: edge.destination,
                weight: edge.weight,
            })
        });
    }
}
