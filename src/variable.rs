#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Module with variable-specific functionality

use *;

/// Trait representing a variable stored in the factor graph.
pub trait Variable : FactorGraphItem {
    /// Get this variable's id.
    fn get_var_id(&self) -> u32;

    /// Add an associated factor to this variable.
    fn add_factor(&mut self, factor: Factor);

    /// Get the factors associated to this variable.
    fn get_factors(&self) -> &Vec<Factor>;
}

/// Struct representing a single variable.
#[derive(Debug)]
pub struct DiscreteVariable<T: std::fmt::Debug + 'static> {
    id: u32,
    name: String,
    factors: Vec<Factor>,
    val_names: Vec<T>,
    domain: Vec<u32>,
}

impl<T: std::fmt::Debug + 'static> DiscreteVariable<T> {
    /// Create a new Variable.
    pub fn new(id: u32, name: &str, val_names: Vec<T>) -> DiscreteVariable<T> {
        let num_names = val_names.len() as u32;
        DiscreteVariable {
            id,
            name: String::from(name),
            factors: vec!(),
            val_names,
            domain: (0..(num_names)).collect()
        }
    }
}

impl<T: std::fmt::Debug + 'static> Variable for DiscreteVariable<T> {
    fn get_var_id(&self) -> u32 {
        self.id.clone()
    }

    fn add_factor(&mut self, factor: Factor) {
        self.factors.push(factor);
    }

    fn get_factors(&self) -> &Vec<Factor> {
        &self.factors
    }
}

impl<T: std::fmt::Debug + Sized> FactorGraphItem for DiscreteVariable<T> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> u32 {
        self.id.clone()
    }

    fn is_factor(&self) -> bool {
        false
    }

    fn add_to_tree(&self, parent_id: u32, tree: &mut SpanningTree) {
        if !tree.has_node(self.id) {
            tree.add_child(parent_id, self.id, &self.name);
        }
    }
}
