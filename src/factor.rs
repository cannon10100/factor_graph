
use *;

type PotentialFunc = fn(&[u32]) -> i32;

/// Struct representing a factor over several variables.
pub struct Factor {
    id: u32,
    variables: Vec<String>,
    func: PotentialFunc,
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
