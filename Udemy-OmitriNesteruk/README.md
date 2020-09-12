# Rust Course

## Cargo Package Manager
 
### Cargo create a project and manage project
This file describe a package to this, the file needs contain the next fields:
    ```
        [package]
        name = "Package Name"
        version = "0.0.1"
        authors = ["Yours names"]
    ```
next to this we need to use:
    ***cargo build***

another form to crete a project using cargo command is:
```Bash
    cargo new <ProjectName> --bin
```
to build or compile a project, only use:
```Bash
    cargo build 
```
to run or compile and run the project use:
```Bash
    cargo run 
```

## Types and Variables
There are different types of data:
+ Boolean (true/false)
+ Integers and decimal numbers
+ Text (individuall character, string of charaters)
+ Structured data ( HTML, XML, JSON, etc...)
+ Binary data (images, proprietary formats )