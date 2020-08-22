# Rust-Course

This Repo will have all my courses on Rust

### To test the Rust installation use:
```Bash
    rustup
```
### To update Rust use:
```Bash
    rustup update
# To see the Rust version
    rustup --version    
```
### To create a new Rust project
```Bash
    cargo new <ProjectName> --bin
```
### To create a compile Rust project
```Bash
    cargo build
```
### To create a run Rust project
```Bash
    cargo run
```
### To download dependecies
```Bash
    cargo fetch

```
### To Compile a File
After using the following command,a binary file is generated, this can be used to run the program
```Bash
    rustc <FileName>.rs
```

## Rust Lang

### Variables
Las variables en rust son inmutables por default, es decir una vez declaradas nose puede cambiar el valor,
esto por la concurrencia y debido a que Rust esta enfocado en seguir la programacion Funcional, ndonde esto es una caracteristica esencial

#### Declaration
```Rust
    let <VarName> = <ValueIsOptional>
```
#### To make variables mutables 
```Rust
    let mut <VarName> = <ValueIsOptional>
```

### Data Types
Rust also use a interpreter of types in build pass, but if we need a especifict type, we can use
 ```Rust  
    let <VarName>:<DataType>
 ```
#### Scalar
+ Integer
    Rust use to default a 32 bits Integer : 
    ```Rust
        i32 
    ```
    but we can also use a 64 bits Integer : 
    ```Rust
        i64
    ```
+ Boolean
  * True
  * False
  * To use 
    ```Rust  
        let <VarName>:bool
    ```
+ Float
    We have two types of float numbers:
    +  ```Rust
            f32
        ```
    +  ```Rust  
        f64 
    ```
+ Characters