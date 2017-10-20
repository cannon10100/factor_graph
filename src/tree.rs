#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Module with spanning tree code

extern crate dot;

use std::io::Write;

/// Struct representing a node in the tree.
#[derive(Clone, Debug)]
pub struct TreeNode {
    // Private fields
    data: u32,
    name: String,
    parent: Option<usize>,

    /// Index of this node within the tree.
    pub index: usize,

    /// Children of this node in the tree.
    pub children: Vec<usize>,
}

/// Struct representing a spanning tree over an underlying factor graph.
#[derive(Debug)]
pub struct SpanningTree {
    // Private fields
    root: usize,

    /// Vector storing all nodes in the tree.
    pub all_nodes: Vec<TreeNode>,

    /// Current index into the vector of all nodes.
    pub cur_index: usize,
}

impl TreeNode {
    /// Make a new tree node.
    pub fn new(index: usize, data: u32, name: &str, parent: usize) -> TreeNode {
        TreeNode {
            index,
            data,
            name: String::from(name),
            parent: Some(parent),
            children: vec!()
        }
    }

    /// Make a new root node, which doesn't have a parent.
    pub fn new_root(index: usize, data: u32, name: &str) -> TreeNode {
        TreeNode {
            index,
            data,
            name: String::from(name),
            parent: None,
            children: vec!()
        }
    }

    /// Function to get this node's name.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Add a child to this tree node.
    pub fn add_child(&mut self, node: usize) {
        self.children.push(node);
    }
}

impl SpanningTree {
    /// Make a new spanning tree.
    pub fn new(root: u32, name: &str, num_nodes: usize) -> SpanningTree {
        let mut node_vec = Vec::with_capacity(num_nodes);
        node_vec.insert(0, TreeNode::new_root(0,root, name));

        SpanningTree {
            root: 0,
            all_nodes: node_vec,
            cur_index: 1
        }
    }

    /// Get the tree node for the input data
    fn get_node_for_data(&self, id: u32) -> Option<usize> {
        for i in 0..self.all_nodes.len() {
            if self.all_nodes[i].data == id {
                return Some(i);
            }
        }

        None
    }

    /// Add a child to the specified node within the tree.
    pub fn add_child(&mut self, parent: u32, child_data: u32, name: &str) {
        let parent_node = match self.get_node_for_data(parent) {
            Some(x) => x,
            None => panic!("Couldn't find input factor graph node in tree")
        };

        let child_node = TreeNode::new(self.cur_index, child_data, name, parent_node);
        self.all_nodes.push(child_node);

        match self.all_nodes.get_mut(parent_node) {
            Some(x) => x.add_child(self.cur_index),
            None => panic!("Parent not found")
        }

        self.cur_index += 1;
    }

    /// Test whether the spanning tree already contains a node
    pub fn has_node(&self, id: u32) -> bool{
        for node in self.all_nodes.iter() {
            if node.data == id {
                return true;
            }
        }

        false
    }

    /// Render this tree to a Graphviz file
    pub fn render_to<W: Write>(&self, output: &mut W) {
        match dot::render(self, output) {
            Ok(_) => println!("Wrote spanning tree graph"),
            Err(_) => panic!("An error occurred writing the spanning tree graph"),
        }
    }
}
