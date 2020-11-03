use crate::graph::{NodePointer, Edge, Graph};
use crate::filesystem::{FileNode, FileType, FileObject, Name, Data};


pub fn ls(directory: &FileNode) -> String {
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


pub fn mkdir(directory: &mut FileNode, name: Name) {
    directory.connect(
        FileNode::create_directory(name, Edge::new())
    );
}


pub fn touch(directory: &mut FileNode, name: Name, data: Data) {
    directory.connect(
        FileNode::create_file(name, data, Edge::new())
    );
}


pub fn write(file: &mut FileNode, input: Data) {
    let n = &mut file.0;

    match n {
        FileType::File{ name: _, data } => { *data = data.to_string() + &input },
        _ => {}
    }
}


pub fn read(file: &FileNode) -> Data {
    let n = &file.value();

    match n {
        FileType::File{ name: _, data } => { data.to_string() },
        _ => { "\0".to_string() }
    }
}


pub fn find(directory: &FileNode, target: Name) -> Result<NodePointer<FileType>, ()> {
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

