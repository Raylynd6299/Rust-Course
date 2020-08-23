fn main() {
    //let m = foo(); // Error porque muere la variable antes de regresar
    let m = bar();

    println!("{}",m)
}

// fn foo() -> &String {
//     let s = String::from("Raymundo");
//     &s;
// }

fn bar() -> String {
    let s = String::from("Raymundo");
    s
}