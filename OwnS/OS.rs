fn main() {
    let a = 20;
    {
        let _b = 19;
    }
    println!("{}",a);
     
    // println!("{}",b) Error fotr the scope

    let st = String::from("Raymundo") ;
    // foo(st)// with this I loss st value
    //But with this i'm not loss the value
    bar(&st);
    print!("{}",st);
}

fn foo(x:String) {
    print!("{}",x);
}
fn bar(x :&String){
    println!("{}",x);
}