
pub mod generic{
    pub fn part1(){
        // making a generic function for finding the largest value in a vector
        // specifying trait bounds with generic data types to ensure the function can work with any type that has the Display and PartialOrd traits
        // PartialOrd trait allows comparison of values
        // PartialEq trait allows equality comparison
        // Clone trait allows cloning of values
        fn greatest<T: std::cmp::PartialEq + std::cmp::PartialOrd + Clone>(items: &Vec<T>) -> T{
            let mut largest = &items[0];
            for item in items{
                if item > largest{
                    largest = item;
                }
            }
            largest.clone()
        } // 
        let numbers = vec![1,2,3,4,5,6,7,8,9,10];
        let result = greatest(&numbers);
        println!("The largest number is {}", result);

        let chars = vec!['a','b','c','d','e','f','g','h','i','j'];
        let result = greatest(&chars);
        println!("The largest char is {}", result);
    }

    pub fn part2(){
        #[derive(Debug)]
        struct Point<T>{
            x: T,
            y: T,
        }
        impl <U> Point<U>{
            fn new(x: U, y: U) -> Self{
                Self{
                    x,
                    y,
                }
            }
            fn x(&self) -> &U{
                &self.x
            }
            fn y(&self) -> &U{
                &self.y
            }
        }
            
        enum Option<T>{
            Some(T),
            None,
        }
        enum Result<T,E>{
            Ok(T),
            Err(E),
        }
        // we can also define methods on structs with generic types
        let p1 = Point{x: 10, y: 20};
        let p2 = Point{x: 10.5, y: 20.5};
        // let p3 = Point{x: "Hello", y: 1}; // this will throw an error because the types are different and the compiler cannot infer the type
        let p3 = Point::new("Hello", "World");
        println!("p1.x = {}", p1.x());
        println!("p2.x = {}", p2.x());
        println!("{:?}",p3)
    }
    pub fn part3(){
        struct Point<T,U>{
            x: T,
            y: U,
        }
        impl <T,U> Point<T,U>{
            fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W>{
                Point{
                    x: self.x,
                    y: other.y,
                }
            } // this function will return a Point<T,W> where T is the type of self.x and W is the type of other.y
        }
        let p1 = Point{x: 10, y: 20.5};
        let p2 = Point{x: "Hello", y: 'c'};
        let p3 = p1.mixup(p2);// this will return a Point<i32,char>

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

}