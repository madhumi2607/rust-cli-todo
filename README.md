# Rust CLI Todo App

A simple command-line Todo application built using **Rust**.

This project demonstrates basic Rust concepts like:

* Command line argument handling
* File I/O
* Task management logic
* Building CLI tools

---

## Features

* Add new tasks
* List all tasks
* Mark tasks as completed
* Delete tasks

---

## Project Structure

```
rust-cli-todo
│
├── Cargo.toml
├── tasks.txt
└── src
    └── main.rs
```

---

## Installation

Clone the repository:

```
git clone https://github.com/madhumi2607/rust-cli-todo.git
```

Move into the project directory:

```
cd rust-cli-todo
```

Build the project:

```
cargo build
```

---

## Usage

### Add a Task

```
cargo run add "Learn Rust"
```

### List Tasks

```
cargo run list
```

Example output:

```
1. [ ] Learn Rust
2. [ ] Build Rust project
```

### Mark Task as Completed

```
cargo run done 1
```

Output:

```
1. [x] Learn Rust
2. [ ] Build Rust project
```

### Delete a Task

```
cargo run delete 2
```

---

## Technologies Used

* Rust
* Cargo (Rust package manager)

---

## Future Improvements

* Add persistent database storage
* Improve CLI command parsing
* Add task priorities and deadlines
* Build a web interface

---

## Author

Madhu
Engineering Student | Software Development Enthusiast
