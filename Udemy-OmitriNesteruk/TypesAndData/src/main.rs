use std::mem;

fn main() {
    //Variable declaration 
    let _var_name:u8 = 123;  

    //Data Types
    let unsigned_bytess:u8 = 123;  // unmutable unsigned 8bits   0 - 255

    let _signed_bytess:i8 = 123;  // unmutable signed 8bits   -128 - 127


    println!("unsignedBytess = {}",unsigned_bytess);

    let mut new_var = 3.1416;

    println!("new_var = {}",new_var);

    new_var = 4.1416;

    println!("new_var = {}",new_var);

    let inter = 32 ; //32-bits  == 4 bytes signed 

    println!("inter = {}, size = {} bytes",inter, mem::size_of_val(&inter));

    let z:isize = 64;
    println!("z = {}, size = {} bytes",z, mem::size_of_val(&z));


    let character:char = 'x';
    let _other_character = 'a';

    println!("character = {}, size = {} bytes",character, mem::size_of_val(&character));

    let stringg = "Raymundo";
    println!("{}",stringg);

    let bools= true;
    println!("bools = {}, size = {} bytes",bools, mem::size_of_val(&bools));

}
