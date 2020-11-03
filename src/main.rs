use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
struct Node<T>(T, Edge<T>);
type Edge<T> = Vec<NodePointerType<T>>;


#[derive(Debug)]
enum NodePointerType<T> {
    NodePointer {
        pointer: Rc<RefCell<Node<T>>>,
    },
    ReadOnlyNodePointer {
        pointer: Rc<Node<T>>,
    },
}


trait Graph<T> {
    fn edge(&self) -> &Edge<T>;
    fn connect(&mut self, node: FileNode);
    fn banish(self);
}


impl Graph<FileType> for FileNode {
    fn edge(&self) -> &Edge<FileType> {
        &self.1
    }

    fn connect(&mut self, node: FileNode) {
        self.1.push(
            NodePointerType::NodePointer{
                pointer: Rc::new(RefCell::new(node))
            }
        );
    }

    fn banish(self) {

    }
}


type Name = String;
type Data = String;


#[derive(Debug)]
enum FileType {
    Directory {
        name: Name,
    },
    File {
        name: Name,
        data: Data,
    }
}


trait FileObject {
    fn name(&self) -> Name;
}


impl FileObject for FileType {
    fn name(&self) -> Name {
        match self {
            FileType::Directory{ name } => { name.to_string() },
            FileType::File{ name, data: _ } => { name.to_string() },
            _ => { "\0".to_string() },
        }
    }
}


type FileNode = Node<FileType>;


impl FileNode {
    fn create_directory(name: Name, edge: Edge<FileType>) -> FileNode {
        Node(
            FileType::Directory {
                name: name,
            },
            edge,
        )
    }

    fn create_file(name: Name, data: Data, edge: Edge<FileType>) -> FileNode {
        Node(
            FileType::File {
                name: name,
                data: data,
            },
            edge,
        )
    }
}


fn ls(directory: &FileNode) -> String {
    let nodes = directory.edge();

    let mut str = match &nodes[0] {
        NodePointerType::NodePointer{ pointer } => {
            pointer.borrow()
                   .0
                   .name()
                   .to_string()
        },
        NodePointerType::ReadOnlyNodePointer{ pointer } => {
            pointer.0
                   .name()
                   .to_string()
        },
    };

    for index in 1..nodes.len() {
        let s = match &nodes[index] {
            NodePointerType::NodePointer{ pointer } => {
                pointer.borrow()
                       .0
                       .name()
                       .to_string()
            },
            NodePointerType::ReadOnlyNodePointer{ pointer } => {
                pointer.0
                       .name()
                       .to_string()
            },
        };
        str = str + "\t" + &s;
    }

    str
}


fn mkdir(directory: &mut FileNode, create_directory_name: Name) {
    directory.connect(
        FileNode::create_directory(create_directory_name, Edge::new())
    );
}


fn main() {
    let mut root = FileNode::create_directory("".to_string(), Edge::new());

    let current = &mut root;

    mkdir(current, "home".to_string());
    mkdir(current, "root".to_string());

    println!("{}", ls(current));
}
