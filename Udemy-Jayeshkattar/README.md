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

#### To print anything in Rust you can use 
In Rust we use ***Macros*** because Rust is focus to functional programming.
the trick to print variables in Rust is use ***{}*** in the place to show the variable
```Rust
    print!("Any text {}",<var>)
```

### Variables
Las variables en rust son inmutables por default, es decir una vez declaradas nose puede cambiar el valor,
esto por la concurrencia y debido a que Rust esta enfocado en seguir la programacion Funcional, donde esto es una caracteristica esencial

Variables in rust are immutable by default, that is, once declared, the value cannot be changed,
This is due to concurrency and because Rust is focused on following Functional programming, where this is an essential feature.

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
  + To use this data type you only need put the letter into simple quotes

#### Compound types
+ Tuples
    Group together a variety of types
    ```Rust
        let <VarName>:(<DataType1>,<DataType2>) = (<Var1>,<Var2>)
    ```
    To access a one index, you need to use (.)
    ```Rust
        let Var = <TupleVar>.<index>
    ```
    To print a tuple you need use ***{:?}*** or ****{:#?}****
    You can see one example in this Repository
+ Array
  Every element must have the same type
  To use:
    ```Rust
        let <VarName> = [<Value>,<Value>, ...]
    ```
  To access a one index, you need to use (.)
    ```Rust
        // The index < len (array)
        let Var = <ArrVar>[<index>]
    ```
### Functions

The functions in Rust is the same like in another programming languages 
```Rust
// Sintax
    fn <FuncName>(<ParmName1>:<DataType>, - - -) -> <DataTypeReturned> {
        return <Value or Variable>
    }
```

### Control Flow 
#### If/Else or If/else if/else
In Ruste we can use if/else or if/else if/else statments to Control the Flow of program
```Rust
// Sintax
    if <statement> <condition> <value> {

    }else {
         
    }
// Or 
    if <statment> <condition> <value> {

    }else if <statement> <condition> <value>  {
         
    }else {

    }

```
Another trick with if in Rustis we can return values out to the if statement
```Rust 
    let <VariableName> = 
        if <condition> {
            // Code
            <ReturnValuewithOutSemicolon>
        }else {
            // Code
            <ReturnValuewithOutSemicolon>
        };
```
Note:Don't forget to put the last semicolon

#### loop 
This Struct is like while in another languages but the only diferrence is that loop not need a condition, in other words with this, we can repeat a block of code any times while the loop can't find a ***break*** statement, and we can continue to the next loop iteration with ***continue***.

```Rust
// Sintax
    loop {
        if a > 18{
            break;
        }
        println!("You're young");
        a = a - 1;
    }
```

loop can return values 
```Rust
    let <varName>  = loop {
        // Code 

        break <ValueOrVariable>;
    }
```
When dealing with nested loops, we need put labels to all the loop with ***'label***.
we can break any loop with ***break <LabelLoop>***
```Rust
    'outer:loop{
        //Loop
        'inner:loop{
            break 'outer;
        }
    }
```

#### while
This is the same like in another languages
```Rust
// Sintax/Example
    let mut var = 0;
    while var < 18 {
        print!("I'm {} years old");
        var = var + 1;
    }
```
#### For loop
This loop is a little different to another languages because in Rust the for statement need a iterator to iterate through 
```Rust
// Sintax
    for <vartoThisScope> in <iterator>{

    }
//Example with a array
    let arr  = [1,2,3,4,5]

    for value in arr.iter {
        //Code
    }
// Example with a range of numbers

    for numb in 1...11{
        //output 1,2,3,4,5,6,7,8,9,10
    }
```

#### Slice
This make us a array part of one origin array
```Rust
//Example 
    let xs:[i64,10] = [0;10];
    println!("{:?}",&xs[0 .. 4]);
```


### Owner Rules
#### ***Three Rules***
+ Each value in Rust has a variable that's called its owner
+ There can only be one owner at a time 
+ When the owner goes out of scope, the value gets droppped

The next text will be in Spanish, if you need translate the text to much information to owner Rules:

En Rust Como bien se explica en las 3 Reglas anteriores, una unica variable puede tener un valor como tal, esta es de las mayores diferencias con otros lenguajes de programacion,
puesto que normalmente cuando pasamos valores a una funcion realmente el valor que tenemos en la funcion es una copia del mismo o en su defecto es una direccion de memoria, y al regresar de la funcion podemos trabajar con el valor como si nada pasara, pues en Rust esto no es de esa forma, una vez nosotros pasamos argumentos a la funcion perdemos los valores que estaban en las variables en el scope origen de estas, por lo que se usa pasar la direccion de las variable para evitar perder esta informacion de las variables originales.

To look one Example in code, click [here](./OwnS/OS.rs)

### Data Races
A data race is similar to a race condition and happens when these three behaviors occur:
+ Two or more pointers access the same data at same time.
+ At least one of the pointers is being used to write to the data.
+ There's no mechanism being used to synchroniza access to data.
  
To look one Example in code, click [here](./RaceCond/RC.rs)

### Structs
+ Custom data Type
+ Make up a meanninful group 
+ Build blocks of creating new data types 
    ```Rust
        struct User {
            username:String,
            email:String,
        }
    //How to use
        let user = User {
            username : String::from("Username"),
            email: String::from("Password"),
        };
    ```
To look a Code Example, click [here](./Structs/example.rs)

### Methods
Functions defined with the context of the structs and First parameter is always &self 
```Rust
    impl Rectangle{
        fn area(&self) -> u32{
            self.width *self.height
        }
    }
```
### Enums
to Enumerations  Enum => (meaning + Data)

To look a Code Example, click [here](./Enums/enum.rs)
#### Options Enum
+ Scenarios where a value could be something or nothing
  
+ Options can handle failure at times, instead of panic 

Sometimes it's desirable to catch the failure of some parts of a program instead of calling panic!; this can be accomplished using the Option enum.

The Option<T> enum has two variants:

None, to indicate failure or lack of value, and
Some(value), a tuple struct that wraps a value with type T.

```Rust
    enum Options<T>{
        Some(T),
        None,
    }
// EXample 
// An integer division that doesn't `panic!`
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            // Failure is represented as the `None` variant
            None
        } else {
            // Result is wrapped in a `Some` variant
            Some(dividend / divisor)
        }
    }
```

### Matching
This statement is like switch statement in another languages,
Compare one value against a series of patterns (This patterns is the powerfull of matching)
```Rust
// Sintax
match <statement> {
    <patter1> => { },
    <pattern2> => {},
    .
    .
    .
}

```
To look a Code Example, click [here](./Matching/Matchh.rs)

### Error Handling
Two Types:
+ Recoverable errors - ***Result<T,E>***
+ Unrecoverable errors - ***panic!***

#### Panic Macro
Bad things happen in code -> Panic
Program prints a failure message, unwinds, clean stack, and quits
Common ocurrence: Bugs
To look a Code Example, click [here](./EH/ErrorHandling.rs)

#### Recoverable Errors
+ Situation wher we can report the error to the user
  + Example : File not found
+ Result enum to rescue!
  ```Rust
    enum Result<T,E> {
        Ok(T),
        Err(E),
    }
  ```