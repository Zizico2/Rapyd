use std::collections::VecDeque;

use syn_rsx::{Node, NodeType};

pub struct DepthFirstIter {
    nodes_stack: Vec<VecDeque<Node>>,
}
impl DepthFirstIter {
    pub fn new(nodes: VecDeque<Node>) -> Self {
        Self {
            nodes_stack: vec![nodes],
        }
    }
}

pub struct DepthFirstIterNode {
    pub node: Node,
    pub level_diff: isize,
}

impl Iterator for DepthFirstIter {
    type Item = DepthFirstIterNode;

    fn next(&mut self) -> Option<Self::Item> {
        let old_stack_len = self.nodes_stack.len();
        let mut nodes = None;
        let mut level_diff = None;
        while let Some(inner_nodes) = self.nodes_stack.last() {
            if !inner_nodes.is_empty() {
                level_diff = Some((self.nodes_stack.len() as isize) - (old_stack_len as isize));
                nodes = self.nodes_stack.last_mut();
                break;
            }
            self.nodes_stack.pop();
        }

        if let Some(nodes) = nodes {
            if let Some(node) = nodes.pop_front() {
                if node.node_type == NodeType::Element {
                    self.nodes_stack.push(node.children.into());
                }
                return Some(DepthFirstIterNode {
                    node: Node {
                        children: vec![],
                        ..node
                    },
                    level_diff: level_diff.expect("level_diff not set!"),
                });
            }
        }
        None
    }
}
