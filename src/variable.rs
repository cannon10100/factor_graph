
use *;


/// Trait representing a variable stored in the factor graph.
pub trait Variable : FactorGraphItem {
    fn add_factor(&mut self, factor: Factor);
    fn get_factors(&self) -> &Vec<Factor>;
    fn get_id(&self) -> u32;
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
