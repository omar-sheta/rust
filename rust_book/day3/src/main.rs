struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,

}
#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{ // by reference to not take ownership
        self.height*self.width
    }
    fn can_hold(&self,other: &Rectangle) ->bool{
        self.width > other.width && self.height> other.height
    }
}
fn  main() {
    let mut user1 = User{
        email: String::from("omarwalaa50@gmail.com"),
        username: String::from("Omar-sheta"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("Amoura");

    let user2 = build_user(
        String::from("youssef@gmail.com"),
        String::from("youssef")
    );

    let user3 = User{
        email: String::from("mariam@gmail.com"),
        username: String::from("mariam"),
        ..user2 // remaining fields get it from user2
    };

    struct Color(i32,i32,i32); // with no name fields
    struct Point(i32,i32,i32);


    test_tuple();
    test_rect();


}
fn test_rect (){
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 200,
        height: 60
    };
    let rect3 = Rectangle{
        width: 20,
        height: 10
    };

    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));

    

    println!("rect: {:#?}",rect1);

    println!("The area of the rectangle is {} square pixels.",rect1.area());
}

fn test_tuple(){
    let rect = (30,50);

    println!("The area of the rectangle is {} square pixels.",area(rect));


}

fn area(dimension: (u32,u32)) ->u32{
    dimension.0 * dimension.1
}



fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1
    }
}