
use std::collections::HashMap;

use serde::Deserialize;

pub struct Config {
    pub root: String,
    pub config_toml: ConfigToml,
}

#[derive(Deserialize)]
pub struct ConfigToml {
    pub package: Package,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub main: Option<String>,
    pub version: Option<String>,
    pub jar_name: Option<String>,
}

pub fn read_toml(root: &str) -> Config {
    let toml = std::fs::read_to_string(format!("{}/geralt.toml", root));
    if toml.is_err() {
        panic!("Failed to read the geralt.toml file: {}", toml.unwrap_err());
    }
    let config: ConfigToml = toml::from_str(&toml.unwrap()).unwrap();
    return Config {
        root: root.to_string(),
        config_toml: config,
    };
}

impl Config {
    pub fn src_path(&self) -> String {
        return format!("{}/src", self.root);
    }

    pub fn jar_path(&self) -> String {
        let cfg = &self.config_toml;
        let name = cfg.package.name.clone();
        let version = cfg.package.version.clone().unwrap_or("0.0.1".to_string());
        let jar_name = format!("{}-{}.jar", name, version);
        let jar_name = cfg.package.jar_name.as_ref().unwrap_or(&jar_name);
        return format!("{}/{}", self.root, jar_name);
    }

    pub fn main_path(&self) -> String {
        let cfg = &self.config_toml;
        let package = &cfg.package;
        let binding = "com/example/Main.java".to_string();
        let main = package.main.as_ref().unwrap_or(&binding);
        return format!("{}/src/{}", self.root, main);
    }

    pub fn main_class_name(&self) -> String {
        let cfg = &self.config_toml;
        let package = &cfg.package;
        let binding = "com/example/Main".to_string();
        let main = package.main.as_ref().unwrap_or(&binding);
        return main.replace("/", ".");
    }

    pub fn target_path(&self) -> String {
        return format!("{}/target", self.root);
    }
}
