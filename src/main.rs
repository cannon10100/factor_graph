#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

extern crate factor_graph;

use factor_graph::FactorGraph;

fn dummy_func(args: &[String]) -> i32 {
    args.len() as i32
}

fn main() {
    let mut graph = FactorGraph::new();

    graph.add_var("first");
    graph.add_var("second");
    graph.add_factor(vec!(String::from("first")), dummy_func);
    graph.add_factor(vec!(String::from("first"), String::from("second")), dummy_func);
    graph.add_factor(vec!(String::from("first")), |args| 0);

    println!("Graph: {:#?}", graph);
}
