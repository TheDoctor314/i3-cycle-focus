use i3ipc::reply::Node;
use std::collections::VecDeque;

#[derive(Debug)]
pub(crate) struct NodeIter<'a> {
    root: &'a Node,
    queue: Option<VecDeque<&'a Node>>,
}

impl<'a> NodeIter<'a> {
    pub(crate) fn new(root: &'a Node) -> Self {
        Self { root, queue: None }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut queue) = self.queue {
            let node = queue.pop_front()?;

            queue.extend(node.nodes.iter());
            queue.extend(node.floating_nodes.iter());

            Some(node)
        } else {
            let mut queue = VecDeque::new();
            queue.extend(self.root.nodes.iter());
            queue.extend(self.root.floating_nodes.iter());

            self.queue = Some(queue);

            Some(self.root)
        }
    }
}
