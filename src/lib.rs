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

pub fn build(root: &str) {
    // Build .class files
    // javac -cp ./src/ -d ./out/ ./src/com/example/Main.java 
    let javac = std::process::Command::new("javac")
        .arg("-cp")
        .arg(format!("{}/src/", root))
        .arg("-d")
        .arg(format!("{}/target/", root))
        .arg(format!("{}/src/com/example/Main.java", root))
        .status();
    if javac.is_err() {
        panic!("Failed to compile the Main.java file: {}", javac.unwrap_err());
    }
    // Build the fat jar
    // jar cvf program.jar -C ./out . 
    let jarcvf = std::process::Command::new("jar")
        .arg("cvf")
        .arg(format!("{}/fat.jar", root))
        .arg("-C")
        .arg(format!("{}/target", root))
        .arg(".")
        .status();
    if jarcvf.is_err() {
        panic!("Failed to create the fat jar: {}", jarcvf.unwrap_err());
    }
    // Set the Main-Class attribute in the manifest file
    // jar cfe program.jar com.example.Main -C ./out . 
    let jarcfe = std::process::Command::new("jar")
        .arg("cfe")
        .arg(format!("{}/fat.jar", root))
        .arg("com.example.Main")
        .arg("-C")
        .arg(format!("{}/target", root))
        .arg(".")
        .status();
    if jarcfe.is_err() {
        panic!("Failed to set the Main-Class attribute: {}", jarcfe.unwrap_err());
    }
}

pub fn run(root: &str) {
    build(root);
    // Run the fat jar
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg(format!("{}/fat.jar", root))
        .status();
    if output.is_err() {
        panic!("Failed to execute the fat jar: {}", output.unwrap_err());
    }
}