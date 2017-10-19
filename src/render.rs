#![deny(missing_docs,
missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces, unused_qualifications)]

//! Rendering functionality for factor_graph

extern crate dot;

use std::borrow::Cow;

type Nd = usize;
type Ed = (usize,usize);

use FactorGraph;
use FactorGraphItem;
use SpanningTree;
use TreeNode;

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

//impl<'a> dot::Labeller<'a, Nd, Ed> for SpanningTree {
//    fn graph_id(&'a self) -> dot::Id<'a> {
//        match dot::Id::new("factor_graph") {
//            Ok(some) => some,
//            Err(_) => panic!("Something went wrong setting graph_id")
//        }
//    }
//
//    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
//        match dot::Id::new(format!("N{}", *n)) {
//            Ok(some) => some,
//            Err(_) => panic!("Node_id failed")
//        }
//    }
//
//    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
//        dot::LabelText::LabelStr(self.all_items[*n].get_name().into())
//    }
//
//    fn node_shape(&'a self, node: &Nd) -> Option<dot::LabelText<'a>> {
//        match self.all_items[*node].is_factor() {
//            true => Some(dot::LabelText::LabelStr("box".into())),
//            false => Some(dot::LabelText::LabelStr("circle".into()))
//        }
//    }
//
//    fn edge_end_arrow(&'a self, _e: &Ed) -> dot::Arrow {
//        dot::Arrow::none()
//    }
//}
//
//impl<'a> dot::GraphWalk<'a, Nd, Ed> for SpanningTree {
//    fn nodes(&self) -> dot::Nodes<'a,Nd> {
//        let mut nodes = Vec::with_capacity(self.cur_index as usize);
//        for i in 0..self.cur_index {
//            nodes.push(i as usize);
//        }
//        nodes.sort();
//        nodes.dedup();
//        Cow::Owned(nodes)
//    }
//
//    fn edges(&'a self) -> dot::Edges<'a, Ed> {
//        let mut edges = vec!();
//        for (_, variable) in &self.variables {
//            for factor in variable.get_factors() {
//                edges.push((variable.get_id() as usize, factor.get_id() as usize));
//            }
//        }
//
//        Cow::Owned(edges)
//    }
//
//    fn source(&self, e: &Ed) -> Nd { let &(s,_) = e; s }
//
//    fn target(&self, e: &Ed) -> Nd { let &(_,t) = e; t }
//}
