fn For_statment(){
    for x in 1..11 {
        println!("{}",x);
    }

    for (pos,y) in (30..41).enumerate(){
        println!("{}:{}",pos,y);
    }
}

fn main () {
    For_statment();
}