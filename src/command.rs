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

    "".to_string()
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


#[cfg(test)]
mod tests {
    use crate::graph::{Edge, Graph};
    use crate::filesystem::{FileNode, FileObject};
    use crate::command::{ls, mkdir, touch, write, read, find};

    #[test]
    fn test_command() {
        let mut root = FileNode::create_directory("".to_string(), Edge::new());
        let current = &mut root;

        mkdir(current, "home".to_string());
        mkdir(current, "root".to_string());
        touch(current, "file1".to_string(), "file1 test".to_string());
        assert_eq!(ls(current), "home\troot\tfile1");

        if let Ok(pointer) = find(current, "file1".to_string()) {
            {
                let node = &pointer.borrow();
                let file = node.value();
                let name = file.name();
                let data = read(node);
                assert_eq!(name, "file1");
                assert_eq!(data, "file1 test");
            }

            {
                let node = &mut pointer.borrow_mut();
                write(node, "\nadd writing".to_string());

                let file = node.value();
                let name = file.name();
                let data = read(node);
                assert_eq!(name, "file1");
                assert_eq!(data, "file1 test\nadd writing");
            }
        }
    }
}

