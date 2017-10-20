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

    /// Add this node to the spanning tree.
    fn add_to_tree(&self, parent_id: u32, tree: &mut SpanningTree);
}

/// Struct representing the full factor graph.
#[derive(Debug)]
pub struct FactorGraph {
    variables: HashMap<String, Box<Variable>>,
    factors: Vec<Factor>,
    next_id: u32,
    all_names: Vec<String>,
    is_factor: Vec<bool>,
}

impl FactorGraph {
    /// Create an empty FactorGraph
    pub fn new() -> FactorGraph {
        FactorGraph {
            variables: HashMap::new(),
            factors: vec!(),
            next_id: 0,
            all_names: vec!(),
            is_factor: vec!(),
        }
    }

    /// Add a new variable with the specified name to the factor graph.
    pub fn add_discrete_var<T : std::fmt::Debug + Clone + 'static>(&mut self, name: &str, val_names: Vec<T>) {
        let new_var = DiscreteVariable::new(self.next_id, name, val_names.clone());

        self.variables.insert(String::from(name),
                              Box::new(new_var));
        self.all_names.insert(self.next_id as usize, String::from(name));
        self.is_factor.insert(self.next_id as usize, false);
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

        self.all_names.insert(self.next_id as usize, String::from(format!("factor<{:?}>", variables.clone())));
        self.is_factor.insert(self.next_id as usize, true);

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
                                                  self.variables.values().len());

        let mut var_iteration = true;
        let mut var_queue: VecDeque<&Box<Variable>> = VecDeque::new();
        let mut factor_queue: VecDeque<&Factor> = VecDeque::new();
        var_queue.push_back(root);

        // BFS through the graph, recording the spanning tree.
        while !var_queue.is_empty() || !factor_queue.is_empty() {
            if var_iteration {
                let node = match var_queue.pop_front() {
                    Some(x) => x,
                    None => panic!("Queue is unexpectedly empty")
                };

                for factor in node.get_factors() {
                    if !spanning_tree.has_node(factor.get_id()) {
                        spanning_tree.add_child((*node).get_id(), factor.get_id(), &factor.get_name());
                        if factor_queue.iter().filter(|x| (*x).get_id() == factor.get_id()).count() == 0 {
                            factor_queue.push_back(factor);
                        }
                    }
                }

                if var_queue.is_empty() {
                    var_iteration = false;
                }
            } else {
                let node = match factor_queue.pop_front() {
                    Some(x) => x,
                    None => panic!("Queue is unexpectedly empty")
                };

                for var_name in node.get_variables() {
                    let var = match self.variables.get(var_name) {
                        Some(x) => x,
                        None => panic!("Could not find variable with name {}", var_name)
                    };

                    if !spanning_tree.has_node(var.get_id()) {
                        spanning_tree.add_child(node.get_id(), var.get_id(), &var.get_name());
                        if var_queue.iter().filter(|x| x.get_id() == var.get_id()).count() == 0 {
                            var_queue.push_back(var);
                        }
                    }
                }

                if factor_queue.is_empty() {
                    var_iteration = true;
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
        graph.add_factor::<i32>(vec!(String::from("second")), dummy_func);
    }

    #[test]
    fn factor_is_added_to_var() {
        let mut graph = FactorGraph::new();

        graph.add_discrete_var("first", vec![1, 2]);
        graph.add_factor::<i32>(vec!(String::from("first")), dummy_func);

        assert_eq!(graph.variables.get("first").unwrap().get_factors()[0].get_variables(),
                   graph.factors[0].get_variables())
    }
}
