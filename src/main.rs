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
type FileNodePointer = NodePointer<FileType>;


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

    fn to_pointer(self) -> FileNodePointer {
        FileNodePointer::new(RefCell::new(self))
    }
}


fn ls(directory: &FileNode) -> String {
    let nodes = directory.edge();
    let mut iter = nodes.iter();

    if let Some(head) = iter.next() {
        let mut str = head.borrow()
            .value()
            .name()
            .to_string();

        iter.for_each(|x| {
            let s = &x.borrow()
                .value()
                .name()
                .to_string();
    
            str = str.to_string() + "\t" + s;
        });
    
        return str;
    }

    "\0".to_string()
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


#[derive(Debug)]
struct Shell {
    root: FileNodePointer,
    current: FileNodePointer,
}


type Buffer = String;
type Arg = str;


fn run(shell: &mut Shell, buffer: &Arg) {
    let argv: Vec<&Arg> = buffer.split(' ').collect();
    let argc = argv.len();
    let mut iter = argv.iter();

    if argc < 1 { return; }

    let command = *iter.next().unwrap();

    if command == ":?" {
        println!("to stop, press Ctrl + c");
        println!("Command list");
        println!("  ls");
        println!("  cd [directory]");
        println!("  find [path]");
        println!("  mkdir [directory]");
        println!("  touch [file]");
        println!("  read [file]");
        println!("  write [file]");
    } else if command == "ls" {
        let current = &shell.current.borrow();
        ls(current);
    }
}


fn interactive() {
    println!("start interactive shell. Enjoy! :/");
    println!("to stop, press Ctrl + c");
    println!("if you need help, type :?");

    let root = FileNode::create_directory("".to_string(), Edge::new()).to_pointer();
    let current = root.clone();
    let shell = &mut Shell{
        root: root,
        current: current,
    };
    
    loop {
        println!("$> ");
        let mut buffer = Buffer::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer == "exit" { break; }

        run(shell, buffer);
    }
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

    interactive();
}
