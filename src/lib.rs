#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

#![doc(html_root_url = "https://cannon10100.github.io/factor_graph/")]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

extern crate dot;

mod render;
pub mod variable;
pub mod factor;
pub mod tree;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;

pub use variable::{Variable, DiscreteVariable};
pub use factor::Factor;
pub use tree::{SpanningTree, TreeNode};

type PotentialFunc = fn(&[u32]) -> i32;

/// Trait representing a generic item stored in the factor graph.
pub trait FactorGraphItem : std::fmt::Debug {
    /// Get the name of this item.
    fn get_name(&self) -> String;

    /// Get this item's id.
    fn get_id(&self) -> u32;

    /// Return whether the item is a factor.
    fn is_factor(&self) -> bool;

    /// Get the neighbors of this item in the factor graph.
    fn get_neighbors(&self, variables: &HashMap<String, Box<Variable>>) -> Vec<u32>;

    /// Add this node to the spanning tree.
    fn add_to_tree(&self, parent_id: u32, tree: &mut SpanningTree);
}

/// Struct representing the full factor graph.
#[derive(Debug)]
pub struct FactorGraph {
    variables: HashMap<String, Box<Variable>>,
    factors: Vec<Factor>,
    all_items: Vec<Box<FactorGraphItem>>,
    next_id: u32,
}

impl FactorGraph {
    /// Create an empty FactorGraph
    pub fn new() -> FactorGraph {
        FactorGraph {
            variables: HashMap::new(),
            factors: vec!(),
            all_items: vec!(),
            next_id: 0
        }
    }

    /// Add a new variable with the specified name to the factor graph.
    pub fn add_discrete_var<T : std::fmt::Debug + Clone + 'static>(&mut self, name: &str, val_names: Vec<T>) {
        self.variables.insert(String::from(name),
                              Box::new(DiscreteVariable::new(self.next_id, name, val_names.clone())));
        self.all_items.insert(self.next_id as usize,
                              Box::new(DiscreteVariable::new(self.next_id, name, val_names)));
        self.next_id += 1;
    }

    /// Add a new factor with the specified variables to the factor graph.
    pub fn add_factor<T: std::fmt::Debug + 'static>(&mut self, variables: Vec<String>, func: PotentialFunc) {
        for var in variables.iter() {
            match self.variables.get_mut(var) {
                Some(var_obj) => {
                    var_obj.add_factor(Factor::new(self.next_id,variables.clone(), func));
                },
                None => panic!("The variable {} was not found in the factor graph.", var)
            }
        }

        self.factors.push(Factor::new(self.next_id, variables.clone(), func));
        self.all_items.insert(self.next_id as usize,
                              Box::new(
                                  Factor::new(self.next_id, variables.clone(), func)));
        self.next_id += 1;
    }

    /// Render this graph to a Graphviz file
    pub fn render_to<W: Write>(&self, output: &mut W) {
        match dot::render(self, output) {
            Ok(_) => println!("Wrote factor graph"),
            Err(_) => panic!("An error occurred writing the factor graph"),
        }
    }

    /// Make a spanning tree from the current factor graph with the specified root variable.
    pub fn make_spanning_tree(&self, var: &str) -> SpanningTree {
        let root = match self.variables.get(var) {
            Some(v) => v,
            None => panic!("Root variable not found")
        };

        let mut spanning_tree = SpanningTree::new(root.get_id(),
                                                  &root.get_name(),
                                                  self.all_items.len());
        let mut queue: VecDeque<&Box<FactorGraphItem>> = VecDeque::new();
        queue.push_back(match self.all_items.get(root.get_id() as usize) {
            Some(y) => y,
            None => panic!("Could not find id in factor graph")
        });

        // BFS through the graph, recording the spanning tree.
        while !queue.is_empty() {
            println!("{:?}", queue);
            let node = match queue.pop_front() {
                Some(x) => x,
                None => panic!("Queue is unexpectedly empty")
            };

            println!("{:?}", node.get_neighbors(&self.variables));

            for n_id in node.get_neighbors(&self.variables) {
                println!("Adding node {}", n_id);
                spanning_tree.add_child((*node).get_id(), n_id, &node.get_name());
                if queue.iter().filter(|x| (*x).get_id() == n_id).count() == 0 {
                    queue.push_back(match self.all_items.get(n_id as usize) {
                        Some(y) => y,
                        None => panic!("Could not find id in factor graph")
                    });
                }
            }
        }

        spanning_tree
    }

    /// Render a spanning tree for the factor graph to the input file, starting from the input variable.
    pub fn render_spanning_tree_to<W: Write>(&self, root_var: &str, output: &mut W) {
        self.make_spanning_tree(root_var).render_to(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_func(args: &[u32]) -> i32 {
        args.len() as i32
    }

    #[test]
    #[should_panic]
    fn factor_with_nonexistent_var() {
        let mut graph = FactorGraph::new();

        graph.add_discrete_var("first", vec![1, 2]);
        graph.add_factor(vec!(String::from("second")), dummy_func);
    }

    #[test]
    fn factor_is_added_to_var() {
        let mut graph = FactorGraph::new();

        graph.add_discrete_var("first", vec![1, 2]);
        graph.add_factor(vec!(String::from("first")), dummy_func);

        assert_eq!(graph.variables.get("first").unwrap().get_factors()[0].get_variables(),
                   graph.factors[0].get_variables())
    }
}
