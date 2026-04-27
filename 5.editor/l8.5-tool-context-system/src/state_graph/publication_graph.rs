use std::collections::{HashMap, VecDeque};

use crate::runtime::state_container_system::StateId;

use super::StateGraph;

impl StateGraph {
    /// Validates that the graph is acyclic.
    ///
    /// Returns Ok(()) if acyclic, or Err with the cycle if one is detected.
    pub fn validate_acyclic(&self) -> Result<(), Vec<StateId>> {
        match self.topological_sort() {
            Ok(_) => Ok(()),
            Err(cycle) => Err(cycle),
        }
    }

    /// Performs topological sort using Kahn's algorithm.
    ///
    /// Returns sorted list if acyclic, or a cycle if one is detected.
    pub fn topological_sort(&self) -> Result<Vec<StateId>, Vec<StateId>> {
        let mut in_degree: HashMap<StateId, usize> = HashMap::new();
        let mut adj_list: HashMap<StateId, Vec<StateId>> = HashMap::new();

        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
            adj_list.insert(node_id.clone(), Vec::new());
        }

        for (from, to) in &self.edges {
            adj_list.get_mut(to).unwrap().push(from.clone());
            *in_degree.get_mut(from).unwrap() += 1;
        }

        let mut queue: VecDeque<StateId> = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut sorted = Vec::new();

        while let Some(node) = queue.pop_front() {
            sorted.push(node.clone());

            if let Some(neighbors) = adj_list.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        if sorted.len() != self.nodes.len() {
            let cycle: Vec<StateId> = in_degree
                .iter()
                .filter(|(_, degree)| **degree > 0)
                .map(|(id, _)| id.clone())
                .collect();
            Err(cycle)
        } else {
            Ok(sorted)
        }
    }
}
