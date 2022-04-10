/// Contains utilities for managing the implication graph
/// of the knowledge base.
/// Adapted from <http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/>

/// Index into [Graph::atoms]
pub type AtomIndex = usize;

/// Index into [Graph::nodes]
pub type NodeIndex = usize;

/// Index into [Graph::edges]
pub type EdgeIndex = usize;

/// Implication Graph
pub struct Graph<T> {
    /// Non-divisible units in the graph
    atoms: Vec<AtomData<T>>,
    /// Connected Atoms
    nodes: Vec<NodeData>,
    /// Connections (implications) between nodes
    edges: Vec<EdgeData>,
}

pub struct AtomData<T> {
    content: T,
}

pub struct NodeData {
    atoms: Vec<AtomIndex>,
    first_outgoing_edge: Option<EdgeIndex>,
}

pub struct EdgeData {
    implies: Vec<NodeIndex>,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl<T: PartialEq> Graph<T> {
    pub fn new() -> Self {
        Self {
            atoms: vec![],
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_atom(&mut self, atom: AtomData<T>) -> usize {
        let index = self.atoms.len();
        self.atoms.push(atom);
        index
    }

    pub fn add_node(&mut self, atoms: Vec<AtomIndex>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            atoms: atoms,
            first_outgoing_edge: None,
        });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, targets: Vec<NodeIndex>) {
        let index = self.edges.len();
        let sourcenode = &mut self.nodes[source];
        self.edges.push(EdgeData {
            implies: targets,
            next_outgoing_edge: sourcenode.first_outgoing_edge,
        });
        sourcenode.first_outgoing_edge = Some(index);
    }
}
