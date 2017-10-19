#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

extern crate dot;

mod render;
mod variable;
mod factor;

use std::collections::HashMap;
use std::io::Write;

use variable::{Variable, DiscreteVariable};
use factor::Factor;

type PotentialFunc = fn(&[u32]) -> i32;

/// Trait representing a generic item stored in the factor graph.
pub trait FactorGraphItem : std::fmt::Debug {
    /// Get the name of this item
    fn get_name(&self) -> String;

    /// Return whether the item is a factor
    fn is_factor(&self) -> bool;
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
                              Box::new(DiscreteVariable::new(self.next_id,name, val_names.clone())));
        self.all_items.insert(self.next_id as usize,
                              Box::new(DiscreteVariable::new(self.next_id,name, val_names)));
        self.next_id += 1;
    }

    /// Add a new factor with the specified variables to the factor graph.
    pub fn add_factor(&mut self, variables: Vec<String>, func: PotentialFunc) {
        for var in variables.iter() {
            match self.variables.get_mut(var) {
                Some(var_obj) => var_obj.add_factor(
                    Factor::new(self.next_id,variables.clone(), func)),
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
