use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq)]
pub struct Node<T>(pub T, pub Edge<T>);
pub type NodePointer<T> = Rc<RefCell<Node<T>>>;
pub type Edge<T> = Vec<NodePointer<T>>;


pub trait Graph {
    type NodeType;
    fn new(value: Self::NodeType, edges: Edge<Self::NodeType>) -> NodePointer<Self::NodeType>;
    fn connect(&mut self, node: NodePointer<Self::NodeType>);
}


impl<T> Graph for Node<T> {
    type NodeType = T;

    fn new(value: T, edges: Edge<T>) -> NodePointer<T> {
        Rc::new(RefCell::new(Node(value, edges)))
    }

    fn connect(&mut self, node: NodePointer<T>) {
        self.1.push(node.clone())
    }
}

