fn main() {
    //Tuples
    let tup:(i32,i32,i64) = (1,2,3);
    let (x,y,z) = tup;

    let a = tup.0;
    println!("Tup is {:#?}",tup);
    println!("x = {}, y = {}, z = {} ",x,y,z);
    println!("a = {}",a);
    
    // Arrays
    let numbers = [1,2,3,4,5,6,7,8,9,0];
    
    println!("index 0 = {}",numbers[0]);
}