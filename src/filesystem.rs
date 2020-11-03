use std::cell::RefCell;
use crate::graph::{Node, NodePointer, Edge, Graph};


pub type Name = String;
pub type Data = String;

pub type FileNode = Node<FileType>;
pub type FileNodePointer = NodePointer<FileType>;


#[derive(Debug)]
pub enum FileType {
    Directory {
        name: Name,
    },
    File {
        name: Name,
        data: Data,
    }
}


pub trait FileObject {
    fn name(&self) -> Name;
}


impl FileObject for FileType {
    fn name(&self) -> Name {
        match self {
            FileType::Directory{ name } => { name.to_string() },
            FileType::File{ name, data: _ } => { name.to_string() },
        }
    }
}


impl FileNode {
    pub fn create_directory(name: Name, edge: Edge<FileType>) -> FileNode {
        Node(
            FileType::Directory {
                name: name,
            },
            edge,
        )
    }

    pub fn create_file(name: Name, data: Data, edge: Edge<FileType>) -> FileNode {
        Node(
            FileType::File {
                name: name,
                data: data,
            },
            edge,
        )
    }

    pub fn to_pointer(self) -> FileNodePointer {
        FileNodePointer::new(RefCell::new(self))
    }
}


impl Graph<FileType> for FileNode {
    fn edge(&self) -> &Edge<FileType> {
        &self.1
    }

    fn value(&self) -> &FileType {
        &self.0
    }

    fn connect(&mut self, node: FileNode) {
        self.1.push(
            NodePointer::new(RefCell::new(node))
        );
    }
}

