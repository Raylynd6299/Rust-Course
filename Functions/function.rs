fn main() {
    foo();
    bar(2);
    let res = sum(2, 4);
    println!("The result is {}",res)
}

fn foo(){
    print!("I'm Raymond");
}
fn bar(x:i32){
    println!("x = {}",x);
}
fn sum(a:i32,b:i32) -> i32 {
    return a + b;
}