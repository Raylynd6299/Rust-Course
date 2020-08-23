use std::fs::File;

fn main() {
    // Panic Error 
    // panic!("Crash and Quit! ");
    // let v =vec![1,2,3];

    // v[99];

    let fi = File::open("hello.txt").expect("We don't have a file yet!");

    // let foo = match fi {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("File not found");
    //     }
    // };

}