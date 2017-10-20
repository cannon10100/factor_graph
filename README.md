# factor_graph [![Build Status](https://travis-ci.org/cannon10100/factor_graph.svg?branch=master)](https://travis-ci.org/cannon10100/factor_graph)
An implementation of probabilistic factor graphs in Rust.

## Status
I'm still sketching out the general design of this library, so it'll be a while before things are up and running for inference. My first goal is to implement the **sum-product** and **max-sum** algorithms for factor graphs composed of only discrete variables.

## Documentation
See the generated Rust documentation [here](http://cannontwo.com/factor_graph).

## Usage
You can compile and run the factor_graph binary with
```
cargo run
```
from the main directory of this repository. Alternatively, there is a provided Dockerfile. You can either build and run this yourself or run `scripts/build_and_run_docker.sh`.

Currently, the executable makes an Ising model using the factor_graph library, then outputs visualizations of the factor graph and a maximal spanning tree to `.dot` files. These can be visualized with Graphviz.
