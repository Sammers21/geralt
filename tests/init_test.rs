use std::{fs::create_dir, path};
use geralt::init;
use geralt::build;

#[test]
fn hello_world_test() {
    // Create a temporary directory 'temp' in the current directory
    let temp = create_dir(path::Path::new("./temp"));
    if temp.is_err() {
        println!("Directory 'temp' already exists, removing it...");
        // If the directory already exists, remove it
        std::fs::remove_dir_all("./temp").unwrap();
        // Create the 'temp' directory again
        create_dir(path::Path::new("./temp")).unwrap();
    }
    // Call the init function with the 'temp' directory
    init("./temp");
    // Check if the 'temp' directory contains the expected files
    assert!(path::Path::new("temp/src/com/example/Main.java").exists());
    assert!(path::Path::new("temp/geralt.toml").exists());
    build(geralt::config::read_toml("./temp"));
    assert!(path::Path::new("temp/hello-world-0.0.1.jar").exists());
    // run java -jar fat.jar  and check the output is "Hello, world!"
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg("temp/hello-world-0.0.1.jar")
        .output()
        .expect("Failed to execute command");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
}

