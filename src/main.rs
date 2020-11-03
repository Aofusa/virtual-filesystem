use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
struct Node<T>(T, Edge<T>);
type NodePointer<T> = Rc<RefCell<Node<T>>>;
type Edge<T> = Vec<NodePointer<T>>;


trait Graph<T> {
    fn edge(&self) -> &Edge<T>;
    fn value(&self) -> &T;
    fn connect(&mut self, node: FileNode);
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

    let mut str = nodes[0].borrow()
        .value()
        .name()
        .to_string();

    for index in 1..nodes.len() {
        let s = &nodes[index].borrow()
            .value()
            .name()
            .to_string();

        str = str + "\t" + s;
    }

    str
}


fn mkdir(directory: &mut FileNode, name: Name) {
    directory.connect(
        FileNode::create_directory(name, Edge::new())
    );
}


fn touch(directory: &mut FileNode, name: Name, data: Data) {
    directory.connect(
        FileNode::create_file(name, data, Edge::new())
    );
}


fn write(file: &mut FileNode, input: Data) {
    let n = &mut file.0;

    match n {
        FileType::File{ name: _, data } => { *data = data.to_string() + &input },
        _ => {}
    }
}


fn read(file: &FileNode) -> Data {
    let n = &file.value();

    match n {
        FileType::File{ name: _, data } => { data.to_string() },
        _ => { "\0".to_string() }
    }
}


fn find(directory: &FileNode, target: Name) -> Result<NodePointer<FileType>, ()> {
    let edges = directory.edge();

    for e in edges {
        let s = e.borrow()
            .value()
            .name()
            .to_string();


        if s == target {
            return Ok(NodePointer::clone(e));
        }
    }

    Err(())
}


fn main() {
    let mut root = FileNode::create_directory("".to_string(), Edge::new());

    let current = &mut root;

    mkdir(current, "home".to_string());
    mkdir(current, "root".to_string());
    touch(current, "file1".to_string(), "file1 test".to_string());
    println!("{}", ls(current));

    if let Ok(pointer) = find(current, "file1".to_string()) {
        {
            let node = &pointer.borrow();
            let file = node.value();
            let name = file.name();
            let data = read(node);
            println!("{}", name);
            println!("{}", data);
        }

        {
            let node = &mut pointer.borrow_mut();
            write(node, "\nadd writing".to_string());

            let file = node.value();
            let name = file.name();
            let data = read(node);
            println!("{}", name);
            println!("{}", data);
        }
    }
}
