/// Contains utilities for managing the implication graph
/// of the knowledge base.
/// Adapted from <http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/>
use std::fmt;

use super::{Atom, Clause, Resolution, UnknownValue, Var};

/// Index into [Graph::atoms]
pub type AtomIndex = usize;

/// Index into [Graph::nodes] pub type NodeIndex = usize;
pub type NodeIndex = usize;

/// Index into [Graph::edges]
pub type EdgeIndex = usize;

/// Implication Graph
pub struct ImplicationGraph {
    /// Non-divisible units in the graph
    pub atoms: Vec<Atom<Var>>,
    /// Connected Atoms
    pub nodes: Vec<NodeData>,
    /// Connections (implications) between nodes
    pub edges: Vec<EdgeData>,
}

#[derive(Debug, PartialEq)]
pub struct NodeData {
    atoms: Vec<AtomIndex>,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, PartialEq)]
pub struct EdgeData {
    implies: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl ImplicationGraph {
    pub fn new() -> Self {
        Self {
            atoms: vec![],
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn find_or_insert_atom(&mut self, atom: Atom<Var>) -> usize {
        match self.atoms.iter().position(|a| a == &atom) {
            Some(index) => index,
            None => {
                let index = self.atoms.len();
                self.atoms.push(atom);
                index
            }
        }
    }

    pub fn find_or_insert_node(&mut self, atoms: Vec<AtomIndex>) -> usize {
        let to_insert = NodeData {
            atoms: atoms,
            first_outgoing_edge: None,
        };

        match self.nodes.iter().position(|node| node == &to_insert) {
            Some(index) => index,
            None => {
                let index = self.nodes.len();
                self.nodes.push(to_insert);
                index
            }
        }
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let index = self.edges.len();
        let sourcenode = &mut self.nodes[source];
        self.edges.push(EdgeData {
            implies: target,
            next_outgoing_edge: sourcenode.first_outgoing_edge,
        });
        sourcenode.first_outgoing_edge = Some(index);
    }

    pub fn add_clause(&mut self, clause: Clause<Var>) {
        let mut previous_index = None;
        for and_chain in clause.0 {
            let mut atom_indices = vec![];
            for atom in and_chain {
                let atom_ix = self.find_or_insert_atom(atom);
                atom_indices.push(atom_ix);
            }
            let node_index = self.find_or_insert_node(atom_indices);
            if let Some(index) = previous_index {
                self.add_edge(index, node_index);
            }
            previous_index = Some(node_index);
        }
    }

    /// Check if the node evaluates to false trivially.
    /// That is the case if one of the atoms is `false`
    fn is_trivial_false(&self, node_index: NodeIndex) -> bool {
        self.nodes[node_index]
            .atoms
            .iter()
            .map(|atom_index| &self.atoms[*atom_index])
            .any(|atom| match atom {
                Atom::Boolean(false) => true,
                _ => false,
            })
    }

    /// Check if the node evaluates to false trivially.
    /// That is the case if all of the atoms are `true`
    fn is_trivial_true(&self, node_index: NodeIndex) -> bool {
        self.nodes[node_index]
            .atoms
            .iter()
            .map(|atom_index| &self.atoms[*atom_index])
            .all(|atom| match atom {
                Atom::Boolean(true) => true,
                _ => false,
            })
    }

    /// Set all elements
    fn set_all_true(&mut self, node_index: NodeIndex) -> Result<bool, ()> {
        let mut modified = false;
        for atom_index in &self.nodes[node_index].atoms {
            match self.atoms[*atom_index] {
                // check for contradictions
                Atom::Boolean(false) => return Err(()),
                Atom::Boolean(true) => {}
                _ => {
                    modified = true;
                    self.atoms[*atom_index] = Atom::Boolean(true)
                }
            }
        }
        Ok(modified)
    }

    fn set_single_false(&mut self, node_index: NodeIndex) -> Result<bool, ()> {
        if self.nodes[node_index].atoms.len() != 1 {
            // Cannot infer values for more than one atom
            return Ok(false);
        }

        let atom_ix = self.nodes[node_index].atoms[0];
        match self.atoms[atom_ix] {
            Atom::Boolean(true) => Err(()),
            Atom::Boolean(false) => Ok(false),
            _ => {
                self.atoms[atom_ix] = Atom::Boolean(false);
                Ok(true)
            }
        }
    }

    pub fn resolution_step(&mut self, resolve_for: AtomIndex) -> Resolution {
        // Check if we are done
        if let Atom::Boolean(b) = &self.atoms[resolve_for] {
            return Resolution::Done(if *b {
                UnknownValue::True
            } else {
                UnknownValue::False
            });
        }

        let mut progressed = false;

        let false_index = self.find_or_insert_atom(Atom::Boolean(false));
        for node_index in 0..self.nodes.len() {
            // if the node is trivially false, replace its contents with a single "false"
            // Note that no knowledge about the other atom's value is gained!
            if self.is_trivial_false(node_index) {
                // we only made progress if the node wasn't just "false" to begin with
                let node = &mut self.nodes[node_index];
                progressed |= node.atoms.len() != 1;
                node.atoms.clear();
                node.atoms.push(false_index);
            }

            // We can remove any non-standalone "true"s
            // but ofc one atom has to remain
            if self.nodes[node_index].atoms.len() != 1 {
                self.nodes[node_index]
                    .atoms
                    .retain(|atom_index| match self.atoms[*atom_index] {
                        Atom::Boolean(true) => false,
                        _ => true,
                    });
            }

            // if the node is true and implies other nodes,
            // all the atoms in those other nodes must be true!
            if self.is_trivial_true(node_index) {
                // TODO prettify, avoid vec allocation
                let successors: Vec<NodeIndex> = self.successors(node_index).collect();
                for successor in successors {
                    match self.set_all_true(successor) {
                        Ok(modified) => progressed |= modified,
                        Err(_) => return Resolution::Contradiction,
                    }
                }
            }

            // If one of the successors is false - then this node must be
            // false as well
            let successor_is_false = self
                .successors(node_index)
                .any(|successor| self.is_trivial_false(successor));
            if successor_is_false {
                // Note that we can only infer the value if the node is a
                // single atom
                match self.set_single_false(node_index) {
                    Ok(modified) => progressed |= modified,
                    Err(_) => return Resolution::Contradiction,
                }
            }
        }

        if progressed {
            Resolution::Progressed
        } else {
            Resolution::Halted
        }
    }

    pub fn successors(&self, node_index: NodeIndex) -> Successors {
        Successors {
            graph: self,
            current_edge_index: self.nodes[node_index].first_outgoing_edge,
        }
    }
}

/// Iterator over the nodes that are implied by a node
pub struct Successors<'graph> {
    graph: &'graph ImplicationGraph,
    current_edge_index: Option<EdgeIndex>,
}

impl Iterator for Successors<'_> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.implies)
            }
            None => None,
        }
    }
}

impl fmt::Debug for ImplicationGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, node) in self.nodes.iter().enumerate() {
            let atoms: Vec<&Atom<Var>> = node.atoms.iter().map(|ix| &self.atoms[*ix]).collect();
            writeln!(f, "{:?}", atoms)?;
            for successor in self.successors(index) {
                let atoms: Vec<&Atom<Var>> = self.nodes[successor]
                    .atoms
                    .iter()
                    .map(|ix| &self.atoms[*ix])
                    .collect();
                writeln!(f, "=> {:?}", atoms)?;
            }
        }
        Ok(())
    }
}
