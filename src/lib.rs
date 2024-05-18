use config::Config;

pub mod config;

pub fn init(root: &str) {
    // Create the dir for main class
    std::fs::create_dir_all(format!("{}/src/com/example", root)).unwrap();
    let jf = std::fs::write(
        format!("{}/src/com/example/Main.java", root),
        "package com.example;

public class Main {
    public static void main(String[] args) {
        System.out.println(\"Hello, world!\");
    }
}",
    );
    if jf.is_err() {
        panic!("Failed to create the Main.java file: {}", jf.unwrap_err());
    }
    let gt = std::fs::write(
        format!("{}/geralt.toml", root),
        "[package]
name = \"hello-world\"
version = \"0.0.1\"

[dependencies]",
    );
    if gt.is_err() {
        panic!("Failed to create the geralt.toml file: {}", gt.unwrap_err());
    }
}

pub fn build(config: Config) {
    // Build .class files
    // javac -cp ./src/ -d ./out/ ./src/com/example/Main.java
    let javac = std::process::Command::new("javac")
        .arg("-cp")
        .arg(config.src_path())
        .arg("-d")
        .arg(config.target_path())
        .arg(config.main_path())
        .status();
    if javac.is_err() {
        panic!(
            "Failed to compile the Main.java file: {}",
            javac.unwrap_err()
        );
    }
    // Build the fat jar
    // jar cvf program.jar -C ./out .
    let jarcvf = std::process::Command::new("jar")
        .arg("cvf")
        .arg(config.jar_path())
        .arg("-C")
        .arg(config.target_path())
        .arg(".")
        .status();
    if jarcvf.is_err() {
        panic!("Failed to create the fat jar: {}", jarcvf.unwrap_err());
    }
    // Set the Main-Class attribute in the manifest file
    // jar cfe program.jar com.example.Main -C ./out .
    let jarcfe = std::process::Command::new("jar")
        .arg("cfe")
        .arg(config.jar_path())
        .arg(config.main_class_name())
        .arg("-C")
        .arg(config.target_path())
        .arg(".")
        .status();
    if jarcfe.is_err() {
        panic!(
            "Failed to set the Main-Class attribute: {}",
            jarcfe.unwrap_err()
        );
    }
}

pub fn run(config: Config) {
    let jar_path = config.jar_path().clone();
    build(config);
    // Run the fat jar
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg(jar_path)
        .status();
    if output.is_err() {
        panic!("Failed to execute the fat jar: {}", output.unwrap_err());
    }
}
