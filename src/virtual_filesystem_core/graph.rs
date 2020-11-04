use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq)]
pub struct Node<T>(pub T, pub Edge<T>);
pub type NodePointer<T> = Rc<RefCell<Node<T>>>;
pub type Edge<T> = Vec<NodePointer<T>>;


pub trait Graph<T> {
    fn edge(&self) -> &Edge<T>;
    fn value(&self) -> &T;
    fn connect(&mut self, node: Node<T>);
}

