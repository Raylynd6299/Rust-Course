
struct Square {
    side: u32,
}

fn main() {

    let square1 = Square{
        side: 10,
    };

    let area = area(&square1);

    println!("Area of square is {}",area);
}

fn area(squaree: &Square) -> u32 {
    squaree.side*squaree.side
}