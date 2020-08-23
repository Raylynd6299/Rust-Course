fn main() {
    struct User{
        username:String,
        email:String,
        password:String,

    };
    let user = User{
        email: String::from("Raylynd6299@github.com"),
        username: String::from("Raylynd6299"),
        password: String::from("passwordd"),
    };
    println!("{}", user.email);
}
