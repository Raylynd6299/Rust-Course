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


#### Declaration
```Rust
    let <VarName> = <ValueIsOptional>
    const <ConstName> = <Value>;
    static <Name> = <Value>;
```
To see a example of static use [Do click here](./staticUse/src/main.rs)
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
        // i8 u8 u16 i16 u32 i32 i64 u64
    ```
    but we can also use a 64 bits Integer : 
    ```Rust
        i64
    ```
    If you need to use the same number of bytes that the SO uses:
    ```Rust
        isize or usize
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

To see more check [This](./TypesAndData/src/main.rs)

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


#### Operators
+ sum -> +
+ res -> -
+ mult -> *
+ div -> /
+ +=
+ -=
+ *=
+ /=
+ %
+ PI = std::f64::consts::PI
+ |  OR
+ &  AND
+ ~  NOT
+ ^  XNOR
+ !  NOR
+ <<
+ >>
+ <
+ >
+ >=
+ <=
+ ==
+ !=


### If statement

is the same like any programming languaje, but with a little extra, to see more uses [check here](.CF_If.rs)

### While statement

is the same like any programming languaje, but with a little extra, to see more uses [check here](./CF_While_loop))

### For statement

is the same like any programming languaje, but with a little extra, to see more uses [check here](./CF_For.rs))

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