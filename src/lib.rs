#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

use std::collections::HashMap;

type PotentialFunc = fn(&[String]) -> i32;

/// Struct representing the full factor graph.
#[derive(Debug)]
pub struct FactorGraph {
    variables: HashMap<String, Variable>,
    factors: Vec<Factor>,
}

/// Struct representing a single variable.
#[derive(Debug)]
pub struct Variable {
    name: String,
    factors: Vec<Factor>,
}

/// Struct representing a factor over several variables.
pub struct Factor {
    variables: Vec<String>,
    func: PotentialFunc,
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
    pub fn add_factor(&mut self, variables: Vec<String>, func: PotentialFunc) {
        for var in variables.iter() {
            match self.variables.get_mut(var) {
                Some(var_obj) => var_obj.add_factor(Factor::new(variables.clone(), func)),
                None => panic!("The variable {} was not found in the factor graph.", var)
            }
        }

        self.factors.push(Factor::new(variables.clone(), func));
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
    pub fn new(variables: Vec<String>, func: PotentialFunc) -> Factor {
        Factor {
            variables,
            func
        }
    }
}

impl std::fmt::Debug for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Factor {{ variables: {:?}, <potential_func: {}> }}",
               self.variables,
               (self.func)(&self.variables))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_func(args: &[String]) -> i32 {
        args.len() as i32
    }

    #[test]
    #[should_panic]
    fn factor_with_nonexistent_var() {
        let mut graph = FactorGraph::new();

        graph.add_var("first");
        graph.add_factor(vec!(String::from("second")), dummy_func);
    }

    #[test]
    fn factor_is_added_to_var() {
        let mut graph = FactorGraph::new();

        graph.add_var("first");
        graph.add_factor(vec!(String::from("first")), dummy_func);

        assert_eq!(graph.variables.get("first").unwrap().factors[0].variables,
                   graph.factors[0].variables)
    }
}
