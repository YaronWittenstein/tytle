use crate::ir::CfgInstruction;
use std::collections::{HashMap, HashSet};

pub type CfgNodeId = usize;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum CfgJumpType {
    WhenTrue,
    Always,
    Fallback,
}

#[derive(Debug, Clone)]
pub enum CfgElement {
    Node(CfgNode),
    Edge(CfgNodeId, CfgNodeId, CfgJumpType),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CfgEdge {
    pub node_id: CfgNodeId,
    pub jmp_type: CfgJumpType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CfgNode {
    pub id: CfgNodeId,
    pub insts: Vec<CfgInstruction>,
    pub incoming: HashSet<CfgEdge>,
    pub outgoing: HashSet<CfgEdge>,
}

impl CfgNode {
    pub fn new(id: CfgNodeId) -> Self {
        Self {
            id,
            insts: Vec::new(),
            incoming: Default::default(),
            outgoing: Default::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.insts.is_empty()
    }

    pub fn is_orphan(&self) -> bool {
        self.incoming.len() == 0 && self.outgoing.len() == 0
    }

    pub fn ends_with_return(&self) -> bool {
        if self.insts.is_empty() {
            return false;
        }

        let last_inst: &CfgInstruction = self.insts.last().unwrap();

        if let CfgInstruction::Return = last_inst {
            return true;
        } else {
            return false;
        }
    }

    pub fn append_inst(&mut self, inst: CfgInstruction) {
        self.insts.push(inst);
    }

    pub fn add_outgoing_edge(&mut self, dst_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.outgoing.insert(CfgEdge {
            node_id: dst_node_id,
            jmp_type,
        });
    }

    pub fn add_incoming_edge(&mut self, src_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.incoming.insert(CfgEdge {
            node_id: src_node_id,
            jmp_type,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CfgGraph {
    next_id: CfgNodeId,
    pub nodes: HashMap<CfgNodeId, CfgNode>,
}

impl CfgGraph {
    pub fn new() -> Self {
        let mut graph = Self {
            nodes: HashMap::new(),
            next_id: 0,
        };

        graph.new_node();

        graph
    }

    pub fn node_is_empty(&self, node_id: CfgNodeId) -> bool {
        let node = self.get_node(node_id);

        node.is_empty()
    }

    pub fn ends_with_return(&self, node_id: CfgNodeId) -> bool {
        let node = self.get_node(node_id);

        node.ends_with_return()
    }

    pub fn is_orphan(&self, node_id: CfgNodeId) -> bool {
        let node = self.get_node(node_id);

        node.is_orphan()
    }

    // used for testing when manually building a graph
    pub fn add_element(&mut self, elem: CfgElement) {
        match elem {
            CfgElement::Node(node) => self.add_node(node),
            CfgElement::Edge(src_id, dst_id, jmp_type) => self.add_edge(src_id, dst_id, jmp_type),
        }
    }

    // used for testing when manually building a graph
    pub fn add_node(&mut self, node: CfgNode) {
        let node_id = node.id;

        self.nodes.insert(node_id, node);

        if self.get_current_id() < node_id {
            self.next_id = node_id + 1;
        }
    }

    // used for testing when manually building a graph
    pub fn add_edge(&mut self, src_id: CfgNodeId, dst_id: CfgNodeId, jmp_type: CfgJumpType) {
        let mut src_node = self.get_node_mut(src_id);
        src_node.add_outgoing_edge(dst_id, jmp_type);

        let mut dst_node = self.get_node_mut(dst_id);
        dst_node.add_incoming_edge(src_id, jmp_type);
    }

    pub fn get_node(&self, node_id: CfgNodeId) -> &CfgNode {
        self.nodes.get(&node_id).unwrap()
    }

    pub fn get_node_mut(&mut self, node_id: CfgNodeId) -> &mut CfgNode {
        self.nodes.get_mut(&node_id).unwrap()
    }

    pub fn current_node_mut(&mut self) -> &mut CfgNode {
        self.get_node_mut(self.next_id - 1)
    }

    pub fn new_node(&mut self) -> CfgNodeId {
        let node = CfgNode::new(self.next_id);

        self.nodes.insert(node.id, node);

        self.next_id += 1;

        self.get_current_id()
    }

    pub fn get_current_id(&self) -> CfgNodeId {
        self.get_next_id() - 1
    }

    pub fn get_next_id(&self) -> CfgNodeId {
        self.next_id
    }

    pub fn get_entry_node_id(&self) -> CfgNodeId {
        0
    }

    pub fn compact(&mut self) {
        let orphan_ids: Vec<usize> = self
            .nodes
            .keys()
            .filter(|&nid| self.is_orphan(*nid))
            .map(|nid| *nid)
            .collect();

        for nid in orphan_ids {
            self.nodes.remove(&nid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfg_graph_node_is_empty() {
        let mut node = CfgNode::new(1);
        assert!(node.is_empty());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(cfg_graph.node_is_empty(1));
    }

    #[test]
    fn cfg_graph_node_is_not_empty() {
        let mut node = CfgNode::new(1);
        node.append_inst(CfgInstruction::Load(1));

        assert!(!node.is_empty());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(!cfg_graph.node_is_empty(1));
    }

    #[test]
    fn cfg_graph_node_ends_with_return() {
        let mut node = CfgNode::new(1);
        node.append_inst(CfgInstruction::Return);
        assert!(node.ends_with_return());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(cfg_graph.ends_with_return(1));
    }

    #[test]
    fn cfg_graph_node_does_not_ends_with_return() {
        let mut node = CfgNode::new(1);
        node.append_inst(CfgInstruction::Load(1));
        assert!(!node.ends_with_return());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(!cfg_graph.ends_with_return(1));
    }

    #[test]
    fn cfg_build_orphan_node() {
        let mut node = CfgNode::new(1);
        assert!(node.is_orphan());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(cfg_graph.is_orphan(1));
    }

    #[test]
    fn cfg_build_node_with_outgoing_edges_is_not_orphan() {
        let mut node = CfgNode::new(1);
        node.add_outgoing_edge(2, CfgJumpType::Always);
        assert!(!node.is_orphan());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(!cfg_graph.is_orphan(1));
    }

    #[test]
    fn cfg_build_node_with_incoming_edges_is_not_orphan() {
        let mut node = CfgNode::new(1);
        node.add_incoming_edge(2, CfgJumpType::Always);
        assert!(!node.is_orphan());

        let mut cfg_graph = CfgGraph::new();
        cfg_graph.add_node(node);

        assert!(!cfg_graph.is_orphan(1));
    }
}
