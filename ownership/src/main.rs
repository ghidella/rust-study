fn main() {
    // In rust, all the variables only exist once and from only one owner, in this case
    // The variable s1 and s2 exists only in the function main. When they go out
    // of scope, rust will automatically free them
    let s1 = String::from("hello");
    // let s2 = s1;

    // For this reason, the code above cannot work. When we declare s2 = s1,
    // We are designating 2 variables that will point to the same memory space,
    // because strings in Rust works like a stack with len and capacity of the string
    // and a pointer to a heap where are stored the string value and index.
    // So when the compiller would free the memory allocated by s2, it will try
    // to free the same memory that was already freed by s1, then we'll have a 
    // double free error. To prevent 
    
    //to avoid that, we can do this instead
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // When we try to do the same thing with integers, unlike strings it works
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // This happens because types such as integers that have a known size at compile
    // time are stored entirely on the stack, so copy them are quick to make.


    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    
}// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

