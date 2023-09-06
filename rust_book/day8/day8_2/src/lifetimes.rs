pub mod lifetime{

    // pub fn part1(){
    //                             // lifetime 'a starts
    //     let r: &i32;            // ------------+-- 'a
    //     {                       //             |
    //     let x = 5;         // -+-- 'b     |
    //         r = &x;             //  |          |
    //     }                       // -+          |
    //     // r now points to invalid data        | 
    //     println!("r: {}", r);   //             |   
    //                             //             |
    // }                           // ------------+

        

    pub fn part2(){
        let x = 5;             // -+-- 'b
        let r = &x;           //  |
        println!("r: {}", r);       //  |
        println!("x: {}", x);       // -+
    }
        // generic lifetime in function signature
        // x, y and return value all have the same lifetime 'a
        // there is a relationship between the lifetime of the reference returned by the longest function and the lifetime of the references passed in.
        // the lifetime of the return value must be the smaller lifetime of the two parameters
        // if x has a smaller lifetime than y, then the return value will have the lifetime of x
        // if y has a smaller lifetime than x, then the return value will have the lifetime of y
        // this makes sense because the reference returned by the longest function will always be a reference to one of the parameters, not something else
        // making it safe to return a reference to one of the parameters
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    pub fn part3(){
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);

        

    //     fn longest(x: &str, y: &str) -> &str {
    //         if x.len() > y.len() {
    //             x
    //         } else {
    //             y
    //         }
    //     }
    // } missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`rustcClick for full compiler diagnostic
    }
    pub fn part4(){
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
        // println!("The longest string is {}", result); will not compile because string2 is out of scope and result is referencing it
    }
    fn return_x<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    pub fn part5(){
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = return_x(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
        println!("The longest string is {}", result); // will compile because string2 is out of scope but result is referencing string1
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    pub fn part6(){
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence }; // lifetime of ImportantExcerpt is the same as the lifetime of first_sentence

        println!("i.part: {}", i.part)
    }

    pub fn part7(){ // lifetime elision
        fn first_word<'a>(s: &'a str) -> &'a str {
            let bytes = s.as_bytes();
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
        
            &s[..]
        }
        /*
            lifetime elision rules:
            1. each parameter that is a reference gets its own lifetime parameter
            2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
            3. if there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters
         */

    }
    pub fn part8(){
        use std::fmt::Display;
        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest_with_an_announcement(string1.as_str(), string2, "Hello, world!");
        println!("The longest string is {}", result);
    }

}