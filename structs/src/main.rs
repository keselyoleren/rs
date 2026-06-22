
struct User {
    username: String,
    email: String,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Reactangle {
    fn can_hold(&self, other: &Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someemail@example.com"),
    };

    user1.username = String::from("anotherusername456");
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("someusername123"), String::from("someemail@example.com"));

    println!("User1: {}, {}", user1.username, user1.email);
    println!("User2: {}, {}", user2.username, user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);

    let subject = AlwaysEqual;
    let another_subject = AlwaysEqual;
    println!("Are they equal? {}", std::ptr::eq(&subject, &another_subject));


    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area((width, height)));
    
    let dim = (20, 30);
    println!("The area of the rectangle is {} square pixels.", new_area(dim));

    let rect1 = Reactangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area((rect1.width, rect1.height)));
    println!("The area of the rectangle is {} square pixels.", rectangle_area(&rect1));

    // Method Syntax

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    let rect2 = Reactangle { width: 20, height: 40 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}



fn build_user(username: String, email: String) -> User {
    User { username, email }
}

fn area((width, height): (u32, u32)) -> u32 {
    width * height
}


fn new_area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn rectangle_area(rect: &Reactangle) -> u32 {
    rect.width * rect.height
}