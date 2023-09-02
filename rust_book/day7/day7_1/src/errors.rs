// file containing module for error handling

pub mod error_handling{
    pub fn part1(){
        panic!("crash and burn"); // panic! macro causes the program to panic and print a message
        // panicing is the same as throwing an exception in other languages
    }

    pub fn part2(){
        fn a(){
            b();
        }
        fn b(){
            c(22);
        }
        fn c(x: i32){
            if x == 22{
                panic!("Don't pass 22!");
            }
        }
        a();
        // the above code will panic and print the message
        // setting BACKTRACE=1 in the environment variables will print the backtrace
    }
    pub fn part3(){
        use std::fs::File;
        use std::io::ErrorKind;
        enum Result<T, E>{ // <T, E> are generic types
            Ok(T),
            Error(E),
        }

        let f = File::open("hello.txt"); // returns a Result<T, E> type

        let f = match f {
            Ok(file) => file, // if the file is found, return the file
            // Err(error) => panic!("Problem opening the file: {:?}", error), // if the file is not found, panic
            Err(error) => match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
                other_error => panic!("Problem opening the file: {:?}", other_error),
                // other_error is a variable that will match any other error
            }
        };
        }
        pub fn part4(){
            use std::fs::File;
            use std::io::ErrorKind;
           
        // handle part3 using closures using unwrap_or_else
        let f = File::open("Hello.txt").unwrap_or_else(|error|
            {
                if error.kind() == ErrorKind::NotFound{
                    File::create("hello.txt").unwrap_or_else(|error|{
                        panic!("Problem creating the file: {:?}", error);
                    })
                }else{
                    panic!("Problem opening the file: {:?}", error);
                }
            });

            // let f = File::open("hello.txt").unwrap().expect("Failed to open hello.txt"); // unwrap() returns the value inside the Ok() variant if it is Ok() 
            // and panics if it is Err() variant and expect() is the same as unwrap() but allows us to specify the panic message

    }

    pub fn part5(){
        use std::fs::File;
        use std::io;
        use std::io::Read;
        fn read_username_from_file() -> Result<String, io::Error>{
            let f = File::open("hello.txt");
            // let f = File::open("hello.txt")?; // ? operator can be used to return the error from the function
            let mut f = match f{
                Ok(file) => file,
                Err(e) => return Err(e),
            };
            let mut s = String::new();
            
            // f.read_to_string(&mut s)?; // ? operator can be used to return the error from the function

            match f.read_to_string(&mut s){
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }

            // Ok(s)// if ? operator is used, this line is not needed as the ? operator will return the value from the function

            /*
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?; // chaining read_to_string() and ? operator so if opening file succeeds, read_to_string() is called
            Ok(s) 
            
             */
        }



    }
    pub fn part6(){
        use std::net::IpAddr;
        let home: IpAddr = "127.0.0.1".parse().unwrap(); // parse() returns a Result<T, E> type and unwrap() returns the value inside the Ok() variant if it is Ok()

        println!("{:?}", home)
    }


}