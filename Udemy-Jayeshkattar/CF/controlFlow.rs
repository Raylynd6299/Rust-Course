fn main() {
    let var  = 10;
    if var > 18 {
        println!("This statment is true")
    }else {
        println!("This statment is false")
    }

    if var % 3 == 0 {
        println!("var can divided by 3 to get zero");
    } else if var % 4 == 0{
        println!("var can divided by 4 to get zero");
    } else if var % 6 == 0 {
         println!("var can divided by 6 to get zero");
    } else {
        println!("var can not divided by 3,4,6 to get zero");
    }
}