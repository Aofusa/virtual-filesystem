extern crate virtual_filesystem_wasm;
use virtual_filesystem_wasm::Cli;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_ls() {
    let shell = &mut Cli::new();

    let buffer = "ls";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "ls a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "ls b c";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "mkdir a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "ls";
    assert_eq!(shell.run(buffer), "a".to_string());

    let buffer = "ls a";
    assert_eq!(shell.run(buffer), "a".to_string());

    let buffer = "ls b c";
    assert_eq!(shell.run(buffer), "a".to_string());
}

#[wasm_bindgen_test]
fn test_pwd() {
    let shell = &mut Cli::new();

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/".to_string());

    let buffer = "pwd a";
    assert_eq!(shell.run(buffer), "/".to_string());

    let buffer = "mkdir a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "cd a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/a".to_string());
}

#[wasm_bindgen_test]
fn test_cd() {
    let shell = &mut Cli::new();

    let buffer = "cd";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "cd a";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "cd b c";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/".to_string());

    let buffer = "cd .";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/".to_string());

    let buffer = "cd ..";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/".to_string());

    let buffer = "mkdir a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "cd a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "cd .";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/a".to_string());

    let buffer = "mkdir b";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "cd b c";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "cd .";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/a/b".to_string());

    let buffer = "cd ..";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/a".to_string());

    let buffer = "cd /";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "pwd";
    assert_eq!(shell.run(buffer), "/".to_string());
}

#[wasm_bindgen_test]
fn test_find() {
    let shell = &mut Cli::new();

    let buffer = "find";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "find a";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "find b c";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "mkdir a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "find a";
    assert_eq!(shell.run(buffer), "a".to_string());

    let buffer = "mkdir b";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "find b c";
    assert_eq!(shell.run(buffer), "b".to_string());
}

#[wasm_bindgen_test]
fn test_mkdir() {
    let shell = &mut Cli::new();

    let buffer = "mkdir";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "mkdir a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "mkdir b c";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "mkdir a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "ls";
    assert_eq!(shell.run(buffer), "a\tb\ta".to_string());
}

#[wasm_bindgen_test]
fn test_touch() {
    let shell = &mut Cli::new();

    let buffer = "touch";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "touch a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "touch b c";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "touch a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "ls";
    assert_eq!(shell.run(buffer), "a\tb\ta".to_string());
}

#[wasm_bindgen_test]
fn test_read() {
    let shell = &mut Cli::new();

    let buffer = "read";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "read a";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "read a b";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "touch a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "read a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "write a 123";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "read a";
    assert_eq!(shell.run(buffer), "123".to_string());

    let buffer = "read a b";
    assert_eq!(shell.run(buffer), "123".to_string());

    let buffer = "mkdir dir";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "read dir";
    assert_eq!(shell.run(buffer), format!("not file."));
}

#[wasm_bindgen_test]
fn test_write() {
    let shell = &mut Cli::new();

    let buffer = "write";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "write a";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "write a 123";
    assert_eq!(shell.run(buffer), format!("not found."));

    let buffer = "touch a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "read a";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "write a";
    assert_eq!(shell.run(buffer), format!("illegal argument."));

    let buffer = "write a 123";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "read a";
    assert_eq!(shell.run(buffer), "123".to_string());

    let buffer = "write a 123 456 xyz";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "read a";
    assert_eq!(shell.run(buffer), "123123 456 xyz".to_string());

    let buffer = "mkdir dir";
    assert_eq!(shell.run(buffer), "".to_string());

    let buffer = "write dir string";
    assert_eq!(shell.run(buffer), format!("not file."));
}

