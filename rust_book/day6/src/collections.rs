
// this file will contain mods for vectors, strings, and hashmaps
    pub mod vectors{
        pub fn part1(){
            let _a = [1,2,3]; // array allocated in stack
            let mut v:Vec<i32> = Vec::new(); // vector allocated in heap
            v.push(1);
            v.push(2);
            v.push(3);

            // vectors can only store values of the same type

            let v2 = vec![1,2,3]; // vector with initial values
            
            println!("v1: {:?}", v);
            println!("v2: {:?}", v2);
        }
        pub fn part2(){
            let v = vec![1,2,3,4,5];
            // reminder you cannot have a mutable and immutable reference to the same variable in the same scope
            let third = &v[2];

            // v.push(6); // this will cause an error because we have an immutable reference to v


            // println!("The third element is {}", third);
            println!("testing with index 20");
            
            match v.get(20){
                Some(third) => println!("The third element is {}", third),
                None => println!("There is no third element."),
            }
            println!("testing with index 2");
            match v.get(2){
                Some(third) => println!("The third element is {}", third),
                None => println!("There is no third element."),
            }
        }
        pub fn part3(){
            let mut v = vec![1,2,3,4,5];

            for i in &mut v{
                *i += 50; // dereference i and add 50 to it
            }
            for i in &v{
                println!("{}", i);
            }
        }

        pub fn part4(){
            enum SpreadSheetCell{
                Int(i32), 
                Float(f64),
                Text(String),
                }

        let row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(10.12),
        ];
        
        match &row[1]{
            SpreadSheetCell::Int(i) => println!("Int: {}", i),
            SpreadSheetCell::Float(f) => println!("Float: {}", f),
            SpreadSheetCell::Text(s) => println!("String: {}", s),
            _ => println!("Not a SpreadSheetCell"),
        }
        }

    }
    /*
            how utf-8 works
                // Bytes
                // word representation as bytes 
                // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135, 224, 165, 135, 224, 164, 164, 224, 165, 135, 224, 164, 184, 224, 165, 141, 224, 164, 168, 224, 164, 174]
                //
                // Scalar values
                // building blocks of a string represent full character or part of a character
                // namaste example
                // ['न', 'म', 'स', '्', 'त', 'े']
                //
                // Grapheme clusters
                // a user-perceived character
                // ["न", "म", "स्", "ते"]
                 
             */
    pub mod strings{
        pub fn part1(){
            let s1 = String::new();
            let s2 = "initial contents";
            let s3 = s2.to_string();
            let s4 = String::from("initial contents");

            let mut s5 = String::from("foo");
            s5.push_str("bar"); 
            // push_str() takes a string slice because we don't want to take ownership of the parameter
            s5.push(char::from('!'));

        }

        pub fn part2(){
            let mut s = String::from("hello");
            s.push(' ');
            s.push_str("bar");
            s.push('!');

            println!("{}", s);
        }

        pub fn part3(){
            let s1 = String::from("Hello, ");
            let s2 = String::from("world!");
            let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used then s2 is borrowed and added to s1
            println!("{}", s3);
        }
        pub fn part4(){
            let hello = String::from("اهلا");
            // let c = hello[0]; 
            // this will not work because rust does not know how many bytes each character in the string is

            // let hello = String::from("Здравствуйте");
            // let answer = &hello[0]; // this will panic because the first byte of the string is not a valid char boundary
     
        }

    }


pub mod hashmaps{ // key and values paired using hashing algorithm
    use std::collections::HashMap;
    pub fn part1(){
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).unwrap(); // returns an option because the key may not exist
    // force unwrap the option to get the value
    println!("score: {:?}", score);
    }

    pub fn part2(){
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("blue"), 50); // this will overwrite the previous value
        scores.entry(String::from("yellow")).or_insert(30); // only insert if the key has no value
        scores.entry(String::from("blue")).or_insert(50); // only insert if the key has no value

        for (key, value) in &scores{
            println!("{}: {}", key, value);
        }
    }

    pub fn part3(){
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_ascii_whitespace(){
            let mut count = map.entry(word).or_insert(0); // returns a mutable reference to the value
            *count += 1;
        }
    }

}

