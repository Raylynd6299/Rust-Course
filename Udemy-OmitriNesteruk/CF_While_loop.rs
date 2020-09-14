fn while_statment(){
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        println!("X = {}",x);
    }
    x=1;
    while x < 1000 {
        x *= 2;
        if x==64 {continue;}
        println!("X = {}",x);
    }
    let mut y = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}",y);

        if y == 1<<20{break;}
    }
}


fn main(){
    while_statment();
}