use std::io;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This line will cause a compile-time error because x is immutable by default
    println!("The value of x is now: {}", x);


    // shadow
    let x = 5;

    let x =  x + 1;
    println!("The value of x is: {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);


    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);


    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {}", guess);

    let x  = 2.0; // f64 by default
    let y: f32 = 3.0; // f32
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);


    // addition
    let sum = 5 + 10;
    println!("The sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient is: {}", quotient);


    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // tupple  
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);


    // array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let third = a[2];
    let fourth = a[3];
    let fifth = a[4];
    println!("The first element is: {}", first);
    println!("The second element is: {}", second);
    println!("The third element is: {}", third);
    println!("The fourth element is: {}", fourth);
    println!("The fifth element is: {}", fifth);


    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The first month is: {}", months[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array a is: {:?}", a);

    check_array_index(a);
    print_label_measurment(5, 'h');
    let x = six();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    if x < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3, or 2");
    }
    
    let is_active = true;
    let x = if is_active { 5 } else { 6 };
    println!("The value of x is: {}", x);

    let mut counter = 0;
    loop {
        println!("The value of counter is: {}", counter);
        counter += 1;

        if counter == 10 {
            break;
        }
    }


'counring_up: loop {
        println!("Enter a number to count up to, or 'exit' to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break 'counring_up;
        }

        let count_to: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number or 'exit'.");
                continue;
            }
        };

        for i in 1..=count_to {
            println!("{}", i);
        }
        
        if count_to >= 10 {
            println!("That's a big number! Let's stop counting.");
            break 'counring_up;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value of the element at index {} is: {}", index, a[index]);
        index += 1;
    }


    for element in a{
        println!("The value of the element is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
}


fn check_array_index(a: [i32; 5]) {
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!");

    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);
}

fn print_label_measurment(value: i32, unit_label: char) {
    println!("The measurment is: {}{}", value, unit_label);
}

fn six() -> i32 {
    6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
