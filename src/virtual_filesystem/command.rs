use crate::virtual_filesystem_core::graph::{NodePointer, Graph};
use crate::virtual_filesystem_core::filesystem::{FileNode, FileType, FileObject, Name, Data};


pub fn ls(directory: &FileNode) -> String {
    let nodes = directory.edge();
    let iter = nodes.iter();
    iter.map(|x| x.borrow().value().name().to_string()).collect::<Vec<String>>().join("\t")
}



pub fn mkdir(directory: &mut FileNode, name: Name) {
    directory.connect(
        FileNode::create_directory(name, vec![])
    );
}


pub fn touch(directory: &mut FileNode, name: Name, data: Data) {
    directory.connect(
        FileNode::create_file(name, data, vec![])
    );
}


pub fn write(file: &mut FileNode, input: &str) -> Result<(), ()> {
    let n = &mut file.0;

    match n {
        FileType::File{ name: _, data } => {
            *data = data.to_string() + &input;
            Ok(())
        },
        _ => { Err(()) }
    }
}


pub fn read(file: &FileNode) -> Result<&Data, ()> {
    let n = &file.value();

    match n {
        FileType::File{ name: _, data } => { Ok(data) },
        _ => { Err(()) }
    }
}


pub fn find(directory: &FileNode, target: &str) -> Result<NodePointer<FileType>, ()> {
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
    use crate::virtual_filesystem_core::graph::{Edge, Graph};
    use crate::virtual_filesystem_core::filesystem::{FileNode, FileObject};
    use crate::virtual_filesystem::command::{ls, mkdir, touch, write, read, find};

    #[test]
    fn test_command() {
        let mut root = FileNode::create_directory("".to_string(), Edge::new());
        let current = &mut root;

        mkdir(current, "home".to_string());
        mkdir(current, "root".to_string());
        touch(current, "file1".to_string(), "file1 test".to_string());
        assert_eq!(ls(current), "home\troot\tfile1");

        if let Ok(pointer) = find(current, "file1") {
            {
                let node = &pointer.borrow();
                let file = node.value();
                let name = file.name();
                let data = read(node);
                assert_eq!(name, "file1");
                assert_eq!(data, Ok(&"file1 test".to_string()));
            }

            {
                let node = &mut pointer.borrow_mut();
                assert_eq!(write(node, "\nadd writing"), Ok(()));

                let file = node.value();
                let name = file.name();
                let data = read(node);
                assert_eq!(name, "file1");
                assert_eq!(data, Ok(&"file1 test\nadd writing".to_string()));
            }
        }
    }
}

