fn main() {
    let var =  String::from("Raymundo");

    println!("Char at position  is {}",
    match var.chars().nth(3) {
        Some(v) => v.to_string(),
        None => "No character at this position".to_string(),
    }
    );
}