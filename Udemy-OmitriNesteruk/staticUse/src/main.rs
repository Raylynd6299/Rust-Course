
const LIFE:i64=153111;
static X: i64 = 255555;
static mut Z:i32 = 132;

fn main() {
    println!("LIFE = {}",LIFE);

    println!("X = {}",X);
    unsafe{
        Z = 777;
        println!("Z = {}",Z);
    }
    
}
