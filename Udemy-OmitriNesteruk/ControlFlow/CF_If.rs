fn if_statment(){
    let temp = 25;
    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really good");
    }else {
        println!("Temp is OK")
    }

    let day = if temp > 20 {"Sunny"} else {"cloudy"};
    println!("Todat is {}",day);

    println!("is it {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    println!("is it {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"OK"});
}


fn main(){
    if_statment();
}