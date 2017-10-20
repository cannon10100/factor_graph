#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Crate allowing creation and manipulation of probabilistic factor graphs.

extern crate factor_graph;
extern crate getopts;

use std::fs::File;
use std::env;

use getopts::Options;

use factor_graph::FactorGraph;

fn dummy_func(args: &[u32]) -> i32 {
    args.len() as i32
}

fn make_ising_model(x_dim: u32, y_dim: u32) -> FactorGraph {
    let mut graph = FactorGraph::new();

    for i in 0..x_dim {
        for j in 0..y_dim {
            graph.add_discrete_var(&format!("({},{})", i, j), vec![0,1]);
        }
    }

    // Add factors between adjacent nodes
    for i in 0..x_dim {
        for j in 0..y_dim {
            if i > 0 {
                graph.add_factor::<i32>(vec!(String::from(format!("({},{})", i - 1, j)),
                                             String::from(format!("({},{})", i, j))), dummy_func);
            }

            if j > 0 {
                graph.add_factor::<i32>(vec!(String::from(format!("({},{})", i, j - 1)),
                                             String::from(format!("({},{})", i, j))), dummy_func);
            }

            if j < y_dim - 1 {
                graph.add_factor::<i32>(vec!(String::from(format!("({},{})", i, j + 1)),
                                             String::from(format!("({},{})", i, j))), dummy_func);
            }

            if i < x_dim - 1 {
                graph.add_factor::<i32>(vec!(String::from(format!("({},{})", i + 1, j)),
                                             String::from(format!("({},{})", i, j))), dummy_func);
            }
        }
    }

    graph
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut x = 10;
    let mut y = 10;

    let mut opts = Options::new();
    opts.optopt("x", "", "set ising model x_dim", "X");
    opts.optopt("y", "", "set ising model y_dim", "Y");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if let Some(thing) = matches.opt_str("x") {
        x = thing.parse().unwrap();
    }
    if let Some(thing) = matches.opt_str("y") {
        y = thing.parse().unwrap();
    }

    // As an example, we build an Ising model
    let graph = make_ising_model(x, y);

//    println!("Graph: {:#?}", graph);
//
//    println!("Spanning tree: {:#?}", graph.make_spanning_tree("(0,0)"));

    let mut graph_file = match File::create("factor_graph.dot") {
        Ok(some) => some,
        Err(_) => panic!("Could not create file")
    };

    let mut spanning_tree_file = match File::create("spanning_tree.dot") {
        Ok(some) => some,
        Err(_) => panic!("Could not create file")
    };

    graph.render_to(&mut graph_file);
    graph.render_spanning_tree_to("(0,0)", &mut spanning_tree_file)
}
