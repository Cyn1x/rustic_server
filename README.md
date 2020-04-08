# Rustic Server

## Table of Contents
1. [Introduction](#Introduction)
2. [Usage](#Usage)
3. [Misc](#Misc)
___
![Demonstration Image of the client interacting with the server](https:// "Demonstration Image")

## Introduction

### What is it?
Rustic Server is a server written in Rust. I originally created this for an assessment at my university.

### What does it do?
Concurrently handles TCP connections and allows clients to play a game of Hangman.

### Purpose
I decided to use Rust to build a server as I was interested in the features available in a modern systems programming language. I do not have an issue with C, or C++, as I enjoy using those languages also.
___

## Usage

### Prerequisites
1. [The `Rust` Programming Language](https://www.rust-lang.org/tools/install).
2. `Cargo`, the Rust package manager.
3. Internet connection so `cargo` can build the required dependencies.

### Building

- If you have to authenticate with a proxy server to access the internet, you must also do this to obtain the required dependencies.

- GUI
1. Download the `.zip` from [my repository](https://github.com/Cyn1x/rustic_server)
2. Extract the contents
3. `cd rustic_server-master`
4. `cargo build`

- Terminal
1. `git clone https://github.com/Cyn1x/rustic_server.git`
2. `unzip rustic_server-master.zip`
3. `cd rustic_server-master`
4. `cargo build`

### Running

#### Running Scripts

This package contains two scripts; `startServer.sh` and `startClient.sh`. To start the server, the port to listen on must be specified as an argument. Additionally, the hostname and port must be given as arguments to start the client.`

1. `chmod u+x startClient.sh` and `chmod u+x startServer.sh` to make the scripts executable.
1. Run `startServer.sh [port]` to start the server
2. Run `startClient.sh [hostname] [port]` to start the client

#### Binary Files

- From the root directory
1. `cargo run --bin server [port]`
2. `cargo run --bin client [hostname] [port]`
___

## Misc

### Tasks
* [x] Server initialisation
* [x] Client initialisation
* [x] Handle server and client communication
* [x] Handle multiple concurrent connections
* [x] Thread pool to limit maximum connections
* [ ] Serve web browsers
* [x] Implement [Hangman](https://en.wikipedia.org/wiki/Hangman_(game) for manual testing
* [x] Shell script to start the server on a given port
* [x] Shell script with arguments to start client and connect to a host on a specific port
* [x] Error handling
* [ ] Client user interface
* [ ] Documentation and comments

### Credits

#### Resources
- The Rust Programming Language [official book](https://doc.rust-lang.org/book/title-page.html)

#### OSS
- The Rust Project Developers for the `rand` [crate](https://crates.io/crates/rand)

Thanks to the Rust Team and all Rust contributors for making the foray into this language so accessible.
