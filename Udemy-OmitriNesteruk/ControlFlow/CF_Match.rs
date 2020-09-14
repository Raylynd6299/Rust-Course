

fn main() {
    let c_c = 10010;
    let country = match c_c {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("The country code with code {} is {} ",c_c,country);
}