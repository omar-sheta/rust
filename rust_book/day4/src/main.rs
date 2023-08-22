enum IPAddrKind {
    V4(String),
    // can be V4(u8,u8,u8,u8)
    V6(String),
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)



}

impl Message{
    fn some_function(){
        println!("Learning Rust!")
    }
}


struct IPAddr{
    kind: IPAddrKind,
    address: String,
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    let localhost = IPAddrKind::V4(String::from("127.0.0.1"));

    let mess = Message::Write(String::from("Hello"));
    Message::some_function();
    // why mess.some_function() not working?
    // because it is not a method, it is an associated function
    // how to use associated function?
    // difference between associated function and method
    // method is defined within the context of a struct
    // associated function is defined within the context of an enum
    // associated function is called using :: syntax


}
fn route(ip_kind: IPAddrKind){}