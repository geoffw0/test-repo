# test-repo
Test repository for fiddling with things like Code Scanning settings.

## Rust Hello World Project

This is a simple Rust "hello world" application with some deliberate vulnerabilities.

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (rustc and cargo)

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing
```bash
cargo test
```

## C++ CodeQL Test Fixture

The C++ source includes examples of code quality issues for CodeQL query testing.

### Prerequisites
- A C++ compiler and `make`

### Building
```bash
make
```

### Running
```bash
make run
```

## Java CodeQL Quality Fixture

The Java source includes labeled code quality examples.

### Prerequisites
- Java 17 and Maven

### Building
```bash
mvn package
```

### Running
```bash
java -cp target/classes Main
```
