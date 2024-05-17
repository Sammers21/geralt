# Geralt

Geralt is a simple Cargo-like build system for Java projects.

## Hello World

You can initialize a new Geralt project by running:

```bash
$ geralt init
```

This will create a new Geralt project in the current directory. The structure of the project will look like this:

```
src/
  com/
    example/
      Main.java
geralt.toml
```

The `Main.java` file will contain the following code:

```java
package com.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

and the `geralt.toml` file will contain the following configuration:

```toml
[package]
name = "hello-world"
version = "0.1.0"

[dependencies]
```

You can build the project by running:

```bash
$ geralt run
```

This will compile the project and run the `Main` class.

You can also build the project without running it by running:

```bash
$ geralt build
```

This will compile the project and create a `target` directory with `hello-world.jar` fat jar file inside.

In order to add dependencies to your project, you can add them to the `geralt.toml` file under the `[dependencies]` section. For example, to add the `org.apache.commons:commons-lang3:3.12.0` dependency, you can add the following line:

```toml
[dependencies]
"org.apache.commons:commons-lang3" = "3.12.0"
```

You can then run `geralt build` to download the dependency and build the project.

## Installation

You can install Geralt by running:

MacOS:

```bash
brew install geralt
```

Linux via sdkman:

```bash
sdk install geralt
```
