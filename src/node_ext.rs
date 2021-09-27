use i3ipc::reply::{Node, NodeType};

use crate::iter::NodeIter;

pub(crate) trait NodeExt {
    fn iter(&self) -> NodeIter<'_>;
    fn find_focused(&self) -> Option<&Self>;
    fn workspaces(&self) -> Vec<&Self>;
    fn leaves(&self) -> Vec<&Self>;
    fn descendants(&self) -> Vec<&Self>;
}

impl NodeExt for Node {
    /// Return the focused container if it exists.
    fn find_focused(&self) -> Option<&Self> {
        self.iter().find(|&node| node.focused)
    }

    /// Return a vector for the workspace containers of this tree.
    fn workspaces(&self) -> Vec<&Self> {
        let mut vec = vec![];

        vec.extend(self.iter().filter(|&node| {
            node.nodetype == NodeType::Workspace
                && node.name.as_ref().map_or(true, |s| !s.starts_with("__"))
        }));

        vec
    }

    /// Return a vector for all the leaf nodes in breadth-first order.
    /// Leaves typically contain the application windows.
    fn leaves(&self) -> Vec<&Self> {
        let mut vec = vec![];

        vec.extend(
            self.iter()
                .filter(|&node| node.nodetype == NodeType::Con && node.nodes.is_empty()),
        );

        vec
    }

    /// Return a vector for all the child nodes in breadth-first order.
    fn descendants(&self) -> Vec<&Self> {
        let mut vec = vec![];
        vec.extend(self.iter());
        vec
    }

    /// Iterate through all the descendants of this node in breadth-first order.
    fn iter(&self) -> NodeIter<'_> {
        NodeIter::new(self)
    }
}
