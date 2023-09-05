// module containing traits

use std::fmt::Display;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String{
        format!("@{}", self.author)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Display for Tweet{ // implement Display trait for Tweet struct
    // this function takes a reference to self and a mutable reference to a formatter and returns a Result
    // the formatter is used to write the formatted string into
    // the formatter is a buffer that this function must write into
    // the formatter will write the formatted string into the buffer and return the result
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{ 
        write!(f, "{}: {}", self.username, self.content)
        // write! macro takes a formatter and a list of arguments
        // the first argument is the formatter
        // the second argument is the first argument to the format! macro
        // the write! macro returns a Result value
        // the write! macro returns an error if the formatted string is not written into the buffer
        // the write! macro returns Ok if the formatted string is written into the buffer
    }
}
impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String{
        format!("@{}", self.username)
    }
}

// define a trait to define shared behaviour between NewsArticle and Tweet
pub trait Summary{
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    // default implementation
    fn summarize_default(&self) -> String{
        format!("Read more from {}...", self.summarize_author())
    }
}

pub fn part1(){
    let tweet = Tweet{
        username: String::from("Omar Sheta"),
        content: String::from("This is a tweet!"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: \n{}", tweet.summarize());
    let article = NewsArticle{
        headline: String::from("This is a headline"),
        location: String::from("This is a location"),
        author: String::from("Omar Sheta"),
        content: String::from("This is content"),
    };
    println!("New article available!\n{}", article.summarize());
    
    println!("Author summarized\n{}", article.summarize_default());
    
    println!("Calling notify function with tweet as argument (trait bound syntax)");
    notify(&tweet);

    // test notify2
    println!("Calling notify2 function with tweet and article as arguments (trait bound syntax)");
    notify2(&tweet, &article);

    // test notify3
    println!("Calling notify3 function with tweet and tweet as arguments (trait bound syntax)");
    notify3(&tweet, &tweet); // works
    // notify3(&tweet, &article); // doesn't work
    

}

pub fn part2(){
    
    println!("{:?}", returns_summarizable().summarize());// this function returns a type that implements the Summary trait
    // how to let "" not printed in the output
}

// pub fn notify(item: &impl Summary){ // trait bound syntax
//     println!("Breaking news! {}", item.summarize()); // we can call summarize because we know that the argument implements the trait

// }

pub fn notify<T: Summary>(item: &T){ // trait bound syntax
    println!("Breaking news! {}", item.summarize()); // we can call summarize because we know that the argument implements the trait
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary){ // trait bound syntax
    println!("Breaking news! {}", item1.summarize()); // we can call summarize because we know that the argument implements the trait
    println!("Breaking news! {}", item2.summarize()); // we can call summarize because we know that the argument implements the trait
} // we can't restrict the types to be the same

pub fn notify3<T: Summary + Display>(item1: &T, item2: &T){ // trait bound syntax
    println!("Breaking news! {}", item1.summarize()); // we can call summarize because we know that the argument implements the trait
    println!("Breaking news! {}", item2.summarize()); // we can call summarize because we know that the argument implements the trait
} // Types restricted to be the same

// to make the function more readable, we can use the where clause
pub fn notify4<T,U>(item1: &T, item2: &U) -> i32
where   T: Summary + Display,
        U: Summary + Display
{ // trait bound syntax

        println!("Breaking news! {}", item1.summarize()); // we can call summarize because we know that the argument implements the trait
        println!("Breaking news! {}", item2.summarize()); // we can call summarize because we know that the argument implements the trait
        0
} // Types restricted to be the same

fn returns_summarizable() -> impl Summary{
    Tweet{
        username: String::from("Omar Sheta"),
        content: String::from("This is a tweet!"),
        reply: false,
        retweet: false,
    }
} // this function returns a type that implements the Summary trait

/*
fn returns_summarizable(switch: bool) -> impl Summary{
    if switch{
        NewsArticle{
            headline: String::from("This is a headline"),
            location: String::from("This is a location"),
            author: String::from("Omar Sheta"),
            content: String::from("This is content"),
        }
    }else{
        Tweet{
            username: String::from("Omar Sheta"),
            content: String::from("This is a tweet!"),
            reply: false,
            retweet: false,
        }
    }
} // this function returns a type that implements the Summary trait
// this function doesn't compile because the compiler doesn't know what type to return 
// because the compiler doesn't know the type of the return value at compile time
// that can be solved in chapter 17 using trait objects 
*/

struct Pair<T>{
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{
            x,
            y,
        }
    }
} // we can implement methods on a generic type
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        }else{
            println!("The largest member is y = {}", self.y);
        }
    }
} // we can implement methods on a generic type only if the type implements a certain trait

impl<T: Display> ToString for T{ // blanket implementation of ToString trait for any type that implements Display trait
    fn to_string(&self) -> String{
        format!("({})", self)
    }
} // we can implement a trait on any type that implements another trait 