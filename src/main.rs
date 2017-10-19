#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

extern crate factor_graph;

use std::fs::File;

use factor_graph::FactorGraph;

fn dummy_func(args: &[u32]) -> i32 {
    args.len() as i32
}

fn main() {
    // As an example, we build an Ising model
    let mut graph = FactorGraph::new();

    for i in 0..10 {
        for j in 0..10 {
            graph.add_discrete_var(&format!("({},{})", i, j), vec![0,1]);
        }
    }

    // Add factors between adjacent nodes
    for i in 0..10 {
        for j in 0..10 {
            if i > 0 {
                graph.add_factor(vec!(String::from(format!("({},{})", i - 1, j)),
                                    String::from(format!("({},{})", i, j))), dummy_func);
            }

            if j > 0 {
                graph.add_factor(vec!(String::from(format!("({},{})", i, j - 1)),
                                    String::from(format!("({},{})", i, j))), dummy_func);
            }

            if j < 9 {
                graph.add_factor(vec!(String::from(format!("({},{})", i, j + 1)),
                                    String::from(format!("({},{})", i, j))), dummy_func);
            }

            if i < 9 {
                graph.add_factor(vec!(String::from(format!("({},{})", i + 1, j)),
                                    String::from(format!("({},{})", i, j))), dummy_func);
            }
        }
    }

    // println!("Graph: {:#?}", graph);
    println!("Spanning tree: {:#?}", graph.make_spanning_tree("(0,0)"));

    let mut f = match File::create("factor_graph.dot") {
        Ok(some) => some,
        Err(_) => panic!("Could not create file")
    };
    graph.render_to(&mut f)
}
