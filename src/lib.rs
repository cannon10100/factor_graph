#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

use std::collections::HashMap;

/// Struct representing the full factor graph.
#[derive(Debug)]
pub struct FactorGraph {
    variables: HashMap<String, Variable>,
    factors: Vec<Factor>,
}

/// Struct representing a single variable.
#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    factors: Vec<Factor>,
}

/// Struct representing a factor over several variables.
#[derive(Debug, Clone, PartialEq)]
pub struct Factor {
    variables: Vec<String>,
}

impl FactorGraph {
    /// Create an empty FactorGraph
    pub fn new() -> FactorGraph {
        FactorGraph {
            variables: HashMap::new(),
            factors: vec!()
        }
    }

    /// Add a new variable with the specified name to the factor graph.
    pub fn add_var(&mut self, name: &str) {
        self.variables.insert(String::from(name), Variable::new(name));
    }

    /// Add a new factor with the specified variables to the factor graph.
    pub fn add_factor(&mut self, variables: Vec<String>) {
        let factor = Factor::new(variables.clone());

        for var in variables.iter() {
            match self.variables.get_mut(var) {
                Some(var_obj) => var_obj.add_factor(factor.clone()),
                None => panic!("The variable {} was not found in the factor graph.", var)
            }
        }

        self.factors.push(factor);
    }
}

impl Variable {
    /// Create a new Variable.
    pub fn new(name: &str) -> Variable {
        Variable {
            name: String::from(name),
            factors: vec!()
        }
    }

    /// Add an associated factor to this variable.
    pub fn add_factor(&mut self, factor: Factor) {
        self.factors.push(factor);
    }
}

impl Factor {
    /// Create a new Factor with associated variables.
    pub fn new(variables: Vec<String>) -> Factor {
        Factor {
            variables,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn factor_with_nonexistent_var() {
        let mut graph = FactorGraph::new();

        graph.add_var("first");
        graph.add_factor(vec!(String::from("second")));
    }

    #[test]
    fn factor_is_added_to_var() {
        let mut graph = FactorGraph::new();

        graph.add_var("first");
        graph.add_factor(vec!(String::from("first")));

        assert_eq!(graph.variables.get("first").unwrap().factors[0], graph.factors[0])
    }
}
