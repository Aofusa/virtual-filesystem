use crate::virtual_filesystem_core::graph::{NodePointer, Graph};
use crate::virtual_filesystem_core::filesystem::{FileNode, FileNodePointer, FileType, FileObject, Name, Data};


pub fn ls(directory: &FileNodePointer) -> String {
    let nodes = &directory.borrow().1;
    let mut iter = nodes.iter();
    let _ = iter.next();
    iter.map(|x| x.borrow().0.name().to_string()).collect::<Vec<String>>().join("\t")
}



pub fn mkdir(directory: &FileNodePointer, name: Name) {
    directory.borrow_mut().connect(
        FileNode::create_directory(name, vec![directory.clone()]).to_pointer()
    );
}


pub fn touch(directory: &FileNodePointer, name: Name, data: Data) {
    directory.borrow_mut().connect(
        FileNode::create_file(name, data, vec![directory.clone()]).to_pointer()
    );
}


pub fn write(file: &FileNodePointer, input: &str) -> Result<(), ()> {
    let n = &mut file.borrow_mut().0;

    match n {
        FileType::File{ name: _, data } => {
            *data = data.to_string() + &input;
            Ok(())
        },
        _ => { Err(()) }
    }
}


pub fn read(file: &FileNodePointer) -> Result<Data, ()> {
    let n = &file.borrow().0;

    match n {
        FileType::File{ name: _, data } => { Ok(data.to_string()) },
        _ => { Err(()) }
    }
}


pub fn find(directory: &FileNodePointer, target: &str) -> Result<NodePointer<FileType>, ()> {
    let edges = &directory.borrow().1;

    for e in edges {
        let s = e.borrow()
            .0
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
    use crate::virtual_filesystem_core::graph::{Graph, Edge};
    use crate::virtual_filesystem_core::filesystem::{FileNode, FileObject};
    use crate::virtual_filesystem::command::{ls, mkdir, touch, write, read, find};

    #[test]
    fn test_command() {
        let root = &FileNode::create_directory("".to_string(), Edge::new()).to_pointer();
        let current = &root.clone();
        root.borrow_mut().connect(current.clone());

        mkdir(current, "home".to_string());
        mkdir(current, "root".to_string());
        touch(current, "file1".to_string(), "file1 test".to_string());
        assert_eq!(ls(current), "home\troot\tfile1");

        if let Ok(pointer) = find(current, "file1") {
            {
                let node = &pointer.borrow();
                let file = &node.0;
                let name = file.name();
                let data = read(&pointer);
                assert_eq!(name, "file1");
                assert_eq!(data, Ok("file1 test".to_string()));
            }

            {
                assert_eq!(write(&pointer, "\nadd writing"), Ok(()));

                let node = &pointer.borrow();
                let file = &node.0;
                let name = file.name();
                let data = read(&pointer);
                assert_eq!(name, "file1");
                assert_eq!(data, Ok("file1 test\nadd writing".to_string()));
            }
        }
    }
}

