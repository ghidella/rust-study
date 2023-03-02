// Silence some warnings
#![allow(unused_variables)]


fn main() {
    let s = String::from("hello world");

    let word = first_word_string(&s); // word will get the value 5

    //s.clear(); // this empties the String, making it equal to ""

    println!("the first word is {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String literal as slice (immutable)
    let s1 = "Hello, world!"; //&str type

    // there are more general slice types like the array slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // &[i32] type

    assert_eq!(slice, &[2, 3]);

}

// Function to return the index of the last character in the first word of a string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to an array of bytes

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}

fn first_word_string(s: &String) -> &str { //string slice
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_improved(s: &str) -> &str { 
    // improves first_word function because allow us to use the same
    // function on both &String values and &str
    s
}


