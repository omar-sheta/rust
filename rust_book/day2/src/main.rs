


fn main() {
    // test1();
    // test2();
    // test3();
    test4();



}

fn test1(){ // test of takes_ownership and makes_copy
    let x = 5; 
    let y = x; //copy

    println!("value of y: {} is copied from x: {} ",y,x);

    let s1 = String::from("hello");
    // let s2 = s1; // Move (not shallow copy)
    let s2 = s1.clone(); // Deep copy (heap data is copied) 
    // println!("{} world",s1 );
    // println!("{} world",s2 );

    let s = String::from("hello");
    // takes_ownership(s); // s's value moves into the function... and so is no longer valid here
    takes_ownership(s.clone()); // clone and pass the value to the function... so that s is still valid here
    println!("{} testing ownership",s); 

    let x = 5; // x comes into scope and of type i32 which is a Copy type

    makes_copy(x);

    println!("{} testing ownership",x); // x is still valid here because i32 is a Copy type

}

fn test2(){
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("{} {} testing ownership",s1,s3); // s3 is still valid here because s3 is moved into s3
}

fn test3(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}",s1,len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);
}
fn test4(){
    // let mut s = String::from("Hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow s (mut reference more than once at a time and at a single scope)

    let mut s = String::from("Hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM (cannot borrow s as mutable because it is also borrowed as immutable)



}
fn test5(){
    // let reference_to_nothing = dangle();

}

fn takes_ownership(some_string : String){
    println!("{}", some_string);
}

fn makes_copy(some_integer : i32){
    println!("{}", some_integer);
}


fn gives_ownership() -> String{
    let some_string = String::from("hello from gives_ownership");
    some_string // return some_string
}

fn takes_and_gives_back(a_string : String) -> String{
    a_string // return a_string
}

fn calculate_length(s : &String) ->  usize{ // s is a reference to a String
    let length = s.len(); // len() returns the length of a String
    length
}

fn change(some_string : &mut String){
    some_string.push_str(", world");
}


// fn dangle() ->&String{
//     let s = String::from("Hello");

//     &s
// }
// q: why is dangle() not working ?
// a: because s is dropped when it goes out of scope, so the reference to it would be pointing to an invalid String

/*
Rules of references
1. At any given time, you can have either one mutable reference 
or any nnumber of immutable references

2. References must always be valid (data pointed to must be valid)
 */