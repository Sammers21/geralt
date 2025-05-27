
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
    let output_str = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    assert_eq!(output_str, "Hello, world!\n");
}

#[test]
fn vertx() {
    let dir = "./examples/vertx";
    let jar = format!("{}/vertx.jar", dir);
    build(geralt::config::read_toml(dir));
    assert!(path::Path::new(&jar).exists());
    // run java -jar vertx.jar  and check the output is "Hello, Vert.x!"
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg(format!("{}/vertx-example.jar", dir))
        .output()
        .expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    assert_eq!(output_str, "Hello, Vert.x!\n");
}

