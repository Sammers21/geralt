
use std::{path};
use geralt::build;

#[test]
fn basic() {
    let dir = "./examples/basic";
    let jar = format!("{}/basic.jar", dir);
    build(geralt::config::read_toml(dir));
    assert!(path::Path::new(&jar).exists());
    // run java -jar fat.jar  and check the output is "Hello, world!"
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg(format!("{}/basic.jar", dir))
        .output()
        .expect("Failed to execute command");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
}

