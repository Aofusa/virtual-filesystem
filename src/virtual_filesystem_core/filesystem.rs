use crate::virtual_filesystem_core::graph::{Node, NodePointer, Edge, Graph};


pub type Name = String;
pub type Data = String;

pub type FileNode = Node<FileType>;
pub type FileNodePointer = NodePointer<FileType>;


#[derive(Debug, PartialEq)]
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
    fn name(&self) -> &Name;
}


impl FileObject for FileType {
    fn name(&self) -> &Name {
        match self {
            FileType::Directory{ name } => { name },
            FileType::File{ name, data: _ } => { name },
        }
    }
}


impl FileNode {
    pub fn create_directory(name: Name, edge: Edge<FileType>) -> FileNodePointer {
        FileNode::new(
            FileType::Directory {
                name: name,
            },
            edge
        )
    }

    pub fn create_file(name: Name, data: Data, edge: Edge<FileType>) -> FileNodePointer {
        FileNode::new(
            FileType::File {
                name: name,
                data: data,
            },
            edge
        )
    }
}


#[cfg(test)]
mod tests_file_object {
    use crate::virtual_filesystem_core::filesystem::{FileType, FileObject};

    #[test]
    fn test_name() {
        let directory = FileType::Directory{ name: "directory".to_string() };
        assert_eq!(directory.name(), "directory");

        let file = FileType::File{ name: "file".to_string(), data: "data".to_string() };
        assert_eq!(file.name(), "file");
    }
}


#[cfg(test)]
mod tests_file_node {
    use crate::virtual_filesystem_core::graph::Graph;
    use crate::virtual_filesystem_core::filesystem::{FileNode, FileType};

    #[test]
    fn test_create() {
        let directory = FileNode::create_directory("directory".to_string(), vec![]);
        assert_eq!(directory, FileNode::new(FileType::Directory{ name: "directory".to_string() }, vec![]));

        let file = FileNode::create_file("file".to_string(), "data".to_string(), vec![]);
        assert_eq!(file, FileNode::new(FileType::File{ name: "file".to_string(), data: "data".to_string() }, vec![]));
    }
}


#[cfg(test)]
mod tests_graph {
    use crate::virtual_filesystem_core::graph::{Node, Edge};
    use crate::virtual_filesystem_core::filesystem::{FileNode, FileType};

    #[test]
    fn test_edge() {
        let node1 = Node(
            FileType::Directory{ name: "node1".to_string() },
            Edge::new(),
        );
        assert_eq!(node1.1, Edge::new());

        let node2 = Node(
            FileType::Directory{ name: "node2".to_string() },
            vec![],
        );
        assert_eq!(node2.1, vec![]);

        let node3 = Node(
            FileType::Directory{ name: "node3".to_string() },
            vec![
                FileNode::create_directory("sub node".to_string(), vec![])
            ],
        );
        assert_eq!(node3.1,
            vec![
                FileNode::create_directory("sub node".to_string(), vec![])
            ]
        );
    }
}

