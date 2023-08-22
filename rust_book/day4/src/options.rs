// enum Option <T>{
//     Some(t),
//     None,
// } //Option is an enum, Some and None are variants of Option for handling
//  the case of having a value and not having a value
fn main(){
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let x = 5;
    let y = Some(5);
    // let sum = x + y;
    // error[E0277]: cannot add `std::option::Option<{integer}>` to `{integer}`

    let sum = x + y.unwrap(); // unwrap() returns the inner value of Some

    match_expression();
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);
}


fn match_expression(){
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1),
        // _ => None, // _ is a catchall value that will match any value that isn't specified before it to prevent errors
    }
}

fn if_let(){
    let some_value = Some(3);
    if let Some(3) = some_value{
        println!("three");
    }
}