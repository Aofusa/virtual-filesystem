mod graph;
mod filesystem;
mod command;
mod shell;


use shell::interactive;


fn main() {
    interactive();
}


#[cfg(test)]
mod tests {
    use crate::graph::{Edge, Graph};
    use crate::filesystem::{FileNode, FileObject};
    use crate::command::{ls, mkdir, touch, write, read, find};

    #[test]
    fn test_run() {
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
