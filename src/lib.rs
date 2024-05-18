pub fn init(dir: &str) {
    // Create the dir for main class
    std::fs::create_dir_all(format!("{}/src/com/example", dir)).unwrap();
    let jf = std::fs::write(
        format!("{}/src/com/example/Main.java", dir),
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
        format!("{}/geralt.toml", dir),
        "[package]
name = \"hello-world\"
version = \"0.0.1\"

[dependencies]",
    );
    if gt.is_err() {
        panic!("Failed to create the geralt.toml file: {}", gt.unwrap_err());
    }
}
