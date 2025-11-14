use crate::graph::graph::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

#[derive(Eq, PartialEq)]
struct PriorityQueueEntry<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    node_id: NodeId,
    distance: u128,
}

impl<NodeId> Ord for PriorityQueueEntry<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // The BinaryHeap returns the largest value first. Since we want the
        // smallest distance value first, we reverse the direction of comparison.
        other.distance.cmp(&self.distance)
    }
}

impl<NodeId> PartialOrd for PriorityQueueEntry<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<NodeId> Graph<NodeId>
where
    NodeId: Eq + Hash + Clone,
{
    pub fn dijkstra(&mut self, start_node: NodeId) -> Result<(), &str>
    where
        NodeId: Eq + Hash + Clone,
    {
        // Initialize start node
        match self.get_mut_node(&start_node) {
            Some(node) => {
                node.min_distance = Some(0);
            }
            None => return Err("The start node does not exist"),
        };

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(PriorityQueueEntry {
            node_id: start_node,
            distance: 0,
        });

        while let Some(PriorityQueueEntry {
            node_id,
            distance: current_distance,
        }) = priority_queue.pop()
        {
            // Skip if the node is already visited
            if self.get_node(&node_id).unwrap().visited {
                continue;
            }

            // Visit the node and read its connected edges
            let destinations = {
                let node = self.get_mut_node(&node_id).unwrap();
                node.visited = true;
                node.destinations.clone()
            };

            // Update distances for all neighbors
            for destination in destinations {
                let neighbor = self.get_mut_node(&destination.node).unwrap();
                if !neighbor.visited {
                    let new_distance = current_distance + destination.weight as u128;
                    match neighbor.min_distance {
                        Some(current_distance) => {
                            if new_distance < current_distance {
                                neighbor.min_distance = Some(new_distance);
                                neighbor.previous_location = vec![node_id.clone()];
                                priority_queue.push(PriorityQueueEntry {
                                    node_id: destination.node,
                                    distance: new_distance,
                                })
                            } else if new_distance == current_distance {
                                neighbor.previous_location.push(node_id.clone());
                            }
                        }
                        None => {
                            neighbor.min_distance = Some(new_distance);
                            neighbor.previous_location = vec![node_id.clone()];
                            priority_queue.push(PriorityQueueEntry {
                                node_id: destination.node,
                                distance: new_distance,
                            })
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn dfs(&mut self, start_node: NodeId) -> Result<(), &str>
    where
        NodeId: Eq + Hash + Clone,
    {
        // Initialize start node
        match self.get_mut_node(&start_node) {
            Some(node) => {
                node.min_distance = Some(0);
            }
            None => return Err("The start node does not exist"),
        };

        let mut stack = Vec::new();
        stack.push((start_node.clone(), 0));

        while let Some((node_id, current_distance)) = stack.pop() {
            // Skip if the node is already visited
            if self.get_node(&node_id).unwrap().visited {
                continue;
            }

            // Visit the node and read its connected edges
            let destinations = {
                let node = self.get_mut_node(&node_id).unwrap();
                node.visited = true;
                node.destinations.clone()
            };

            // Update distances for all neighbors
            for destination in destinations {
                let neighbor = self.get_mut_node(&destination.node).unwrap();
                if !neighbor.visited {
                    let new_distance = current_distance + destination.weight as u128;
                    match neighbor.min_distance {
                        Some(current_distance) => {
                            if new_distance < current_distance {
                                neighbor.min_distance = Some(new_distance);
                                neighbor.previous_location = vec![node_id.clone()];
                            } else if new_distance == current_distance {
                                neighbor.previous_location.push(node_id.clone());
                            }
                        }
                        None => {
                            neighbor.min_distance = Some(new_distance);
                            neighbor.previous_location = vec![node_id.clone()];
                        }
                    }
                    stack.push((destination.node, new_distance));
                }
            }
        }

        Ok(())
    }

    pub fn get_shortest_path(&self, finish: &NodeId) -> Option<Vec<NodeId>> {
        let mut route: Vec<NodeId> = vec![finish.clone()];
        let next_tiles = &self.get_node(&finish)?.previous_location;

        if next_tiles.len() == 0 {
            return Some(route);
        }

        for tile in next_tiles {
            if let Some(value) = &mut self.get_shortest_path(&tile) {
                route.append(value);
                return Some(route);
            }
        }

        return None;
    }

    pub fn get_shortest_paths(&self, finish: &NodeId) -> Vec<Vec<NodeId>> {
        let backward_paths = self.find_all_paths_helper(finish);

        backward_paths
            .into_iter()
            .map(|mut path| {
                path.reverse();
                path
            })
            .collect()
    }

    fn find_all_paths_helper(&self, current: &NodeId) -> Vec<Vec<NodeId>> {
        if let Some(current_node) = self.get_node(current) {
            if current_node.previous_location.is_empty() {
                return vec![vec![current.clone()]];
            }

            let mut all_paths_from_here = Vec::new();

            for prev_node_id in &current_node.previous_location {
                let paths_to_predecessor = self.find_all_paths_helper(prev_node_id);

                for mut path in paths_to_predecessor {
                    path.push(current.clone());
                    all_paths_from_here.push(path);
                }
            }

            all_paths_from_here
        } else {
            Vec::new()
        }
    }
}
