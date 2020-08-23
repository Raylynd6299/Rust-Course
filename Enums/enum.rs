enum IpVariants{
    IPV4(String),
    IPV6(String),
}
enum Message {
    Write(String),
    Color(u8,u8,u8),
    Move{x:i32,y:i32},
    Quit,
}
impl Message {
    fn call(&self) {
        println!("I'm inside the impl");
    }
}

fn main() {
    let _ip1 = IpVariants::IPV4(String::from("192.168.100.1"));
    let _ip2 = IpVariants::IPV6(String::from("::1"));

    let varr = Message::Write(String::from("Hello"));
    
    varr.call();
}