package com.example.vertx;

import io.vertx.core.Vertx;

public class Main {
    public static void main(String[] args) {
        Vertx vertx = Vertx.vertx();
        var fs = vertx.fileSystem();
        fs.writeFileBlocking("example.txt", "Hello, Vert.x!".getBytes());
        System.out.println("File written successfully!");

    }
}
