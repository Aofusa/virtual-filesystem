use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq)]
pub struct Node<T>(pub T, pub Edge<T>);
pub type NodePointer<T> = Rc<RefCell<Node<T>>>;
pub type Edge<T> = Vec<NodePointer<T>>;


pub trait Graph {
    type NodeType;
    fn connect(&mut self, node: NodePointer<Self::NodeType>);
}

