use std::{collections::HashMap, hash::Hash};

use slab_tree::{NodeId, RemoveBehavior, Tree};

use super::env::Environments;

/// An implementation of `Environments` with an in-memory `Tree`.
pub struct EnvironmentTree<Var, Val> {
    root_id: NodeId,
    tree: Tree<HashMap<Var, Val>>,
}

impl<Val, Var> EnvironmentTree<Val, Var> {
    pub fn empty(capacity: usize) -> Self {
        let mut tree = Tree::new();
        let root_id = tree.set_root(HashMap::with_capacity(capacity));
        EnvironmentTree {
            root_id: root_id,
            tree: tree,
        }
    }
}

impl<Var: Eq + Hash, Val> Environments for EnvironmentTree<Var, Val> {
    type Env = NodeId;
    type Val = Val;
    type Var = Var;

    fn root(&self) -> Self::Env {
        self.root_id
    }

    fn has_children(&self, at: Self::Env) -> bool {
        self.tree
            .get(at)
            .map(|n| n.first_child().is_some())
            .unwrap_or(false)
    }

    fn drop(&mut self, at: Self::Env) {
        self.tree.remove(at, RemoveBehavior::DropChildren);
    }

    fn push(&mut self, at: Self::Env, capacity: usize) -> Option<Self::Env> {
        self.tree
            .get_mut(at)
            .map(|mut n| n.append(HashMap::with_capacity(capacity)).node_id())
    }

    fn get(&self, at: Self::Env, x: &Var) -> Option<&Val> {
        let mut current = Some(at);
        while let Some(node) = current.and_then(|c| self.tree.get(c)) {
            if let Some(v) = node.data().get(&x) {
                return Some(v);
            }

            current = node.parent().map(|n| n.node_id());
        }

        None
    }

    fn define(&mut self, at: Self::Env, x: Var, v: Val) -> Result<(), (Var, Val)> {
        match self.tree.get_mut(at) {
            None => Err((x, v)),
            Some(mut n) => {
                if n.data().contains_key(&x) {
                    Err((x, v))
                } else {
                    n.data().insert(x, v);
                    Ok(())
                }
            }
        }
    }
}
