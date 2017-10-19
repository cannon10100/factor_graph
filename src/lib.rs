#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

extern crate dot;

use std::collections::HashMap;
use std::borrow::Cow;
use std::io::Write;

type Nd = usize;
type Ed = (usize,usize);

type PotentialFunc = fn(&[u32]) -> i32;

/// Trait representing a variable stored in the factor graph.
trait Variable : FactorGraphItem {
    fn add_factor(&mut self, factor: Factor);
    fn get_factors(&self) -> &Vec<Factor>;
    fn get_id(&self) -> u32;
}

/// Trait representing a generic item stored in the factor graph.
trait FactorGraphItem : std::fmt::Debug {
    fn get_name(&self) -> String;
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

/// Struct representing a single variable.
#[derive(Debug)]
pub struct DiscreteVariable<T: std::fmt::Debug + 'static> {
    id: u32,
    name: String,
    factors: Vec<Factor>,
    val_names: Vec<T>,
    domain: Vec<u32>,
}

/// Struct representing a factor over several variables.
pub struct Factor {
    id: u32,
    variables: Vec<String>,
    func: PotentialFunc,
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

impl<'a> dot::Labeller<'a, Nd, Ed> for FactorGraph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        match dot::Id::new("factor_graph") {
            Ok(some) => some,
            Err(_) => panic!("Something went wrong setting graph_id")
        }
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        match dot::Id::new(format!("N{}", *n)) {
            Ok(some) => some,
            Err(_) => panic!("Node_id failed")
        }
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.all_items[*n].get_name().into())
    }

    fn node_shape(&'a self, node: &Nd) -> Option<dot::LabelText<'a>> {
        match self.all_items[*node].is_factor() {
            true => Some(dot::LabelText::LabelStr("box".into())),
            false => Some(dot::LabelText::LabelStr("circle".into()))
        }
    }

    fn edge_end_arrow(&'a self, _e: &Ed) -> dot::Arrow {
        dot::Arrow::none()
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for FactorGraph {
    fn nodes(&self) -> dot::Nodes<'a,Nd> {
        let mut nodes = Vec::with_capacity(self.next_id as usize);
        for i in 0..self.next_id {
            nodes.push(i as usize);
        }
        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let mut edges = vec!();
        for (_, variable) in &self.variables {
            for factor in variable.get_factors() {
                edges.push((variable.get_id() as usize, factor.get_id() as usize));
            }
        }

        Cow::Owned(edges)
    }

    fn source(&self, e: &Ed) -> Nd { let &(s,_) = e; s }

    fn target(&self, e: &Ed) -> Nd { let &(_,t) = e; t }
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
            domain: (0..(num_names + 1)).collect()
        }
    }
}

impl<T: std::fmt::Debug + 'static> Variable for DiscreteVariable<T> {
    /// Add an associated factor to this variable.
    fn add_factor(&mut self, factor: Factor) {
        self.factors.push(factor);
    }

    fn get_id(&self) -> u32 {
        self.id.clone()
    }

    fn get_factors(&self) -> &Vec<Factor> {
        &self.factors
    }
}

impl<T: std::fmt::Debug> FactorGraphItem for DiscreteVariable<T> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_factor(&self) -> bool {
        false
    }
}

impl Factor {
    /// Create a new Factor with associated variables.
    pub fn new(id: u32, variables: Vec<String>, func: PotentialFunc) -> Factor {
        Factor {
            id,
            variables,
            func
        }
    }

    /// Function to get a Factor's id
    pub fn get_id(&self) -> u32 {
        self.id.clone()
    }

    /// Function to get variables associated with this factor
    pub fn get_variables(&self) -> &Vec<String> {
        &self.variables
    }
}

impl std::fmt::Debug for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Factor {{ variables: {:?}, <potential_func> }}",
               self.variables) // TODO: actually pass in values
    }
}

impl FactorGraphItem for Factor {
    fn get_name(&self) -> String {
        format!("factor<{:?}>", self.variables)
    }

    fn is_factor(&self) -> bool {
        true
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
