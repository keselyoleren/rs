fn main() {
    println!("--- scop and drop ---");
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
        println!("The value of s is: {}", s);
    } // this scope is now over, and s is no longer valid
 

    println!("--- move ---");
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, and s1 is no longer valid
    // println!("The value of s1 is: {}", s1); // this will cause a compile-time error
    println!("The value of s2 is: {}", s2); // this will work because s2 is valid
    // println!("The value of s2 is: {}", s1); // this will cause a compile-time error because s1 is no longer valid


    println!("--- clone ---");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is cloned to s2, and both s1 and s2 are valid
    println!("The value of s1 is: {}", s1); // this will work because s1 is still valid
    println!("The value of s2 is: {}", s2); // this will work because s2 is valid

    println!("--- copy ---");
    let x = 5; // x is an integer, which is a type that implements the Copy trait
    let y = x; // x is copied to y, and both x and y are valid
    println!("The value of x is: {}", x); // this will work because x is still valid
    println!("The value of y is: {}", y); // this will work because y is valid


    println!("--- ownership and functions ---");
    let s = String::from("hello"); // s is valid from this point forward
    takes_ownership(s); // s's value moves into the function, and s is no longer valid here
    // println!("The value of s is: {}", s); // this will cause a compile-time error because s is no longer valid

    let x = 5; // x is valid from this point forward
    makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
    println!("The value of x is: {}", x); // this will work because x is still valid

    let s1 = String::from("hello");
    let s2 = get_string(s1); // s2 is valid from this point forward
    println!("The value of s2 is: {}", s2); // this will work because

    println!("-- borrowing -- "); 
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // we pass a reference
    println!("The length of '{}' is {}.", s1, len); // this will work because s1 is still valid



    println!("--- slice ---");
    let s = String::from("hello world");
    let hello = &s[0..5]; // hello is a slice of s, and it is valid from this point forward
    let world = &s[6..11]; // world is a slice of s, and it is valid from this point forward
    println!("The value of hello is: {}", hello); // this will work because hello is valid
    println!("The value of world is: {}", world); // this will work because world is valid

    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..3]; // slice is a slice of array, and it is valid from this point forward
    println!("The value of slice is: {:?}", slice); // this will work because slice is valid    

}

fn get_string(s: String) -> String {
    s // s is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) {
    println!("The value of some_string is: {}", some_string);
} // some_string goes out of scope and is dropped here

fn makes_copy(some_integer: i32) {
    println!("The value of some_integer is: {}", some_integer);
} // some_integer goes out of scope here, but nothing special happens because i32 is Copy

fn calculate_length(s: &String) -> usize {
    s.len() // we return the length of the string, which is a usize
} // s goes out of scope here, but because it does not have ownership of what it refers to, it is not dropped   