use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;


#[test]
#[should_panic(expected = "No path given")]
fn file_not_given() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.unwrap();
}

#[test]
#[should_panic(expected = "Should have been able to read the file")]
fn file_not_found() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("test/file/not/found");
    cmd.unwrap();
}

#[test]
fn hello_world_run_correctly() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("brainfuck-examples/hello_world.bf");
    let predicate = predicate::str::contains("Hello World!");
    cmd.assert().success().stdout(predicate);
}