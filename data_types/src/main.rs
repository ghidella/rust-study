fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{} {}", quotient, truncated);

    // remainder
    let remainder = 43 % 5;

    //tuple (different types)
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; //descontructuring

    //accessing tuple elements
    let five_hundred = x;
    let six_point_four = tup.1;

    //char type
    let c = 'z';
    let z: char = 'Z'; // expliciting type
    let hear_eyed_cat = '😻'; //rust char type represent Unicode Scalar Value

    let a: f64 = 123123.1231233123;
    println!("{a}");

    //arrays
    let a = [1, 2, 3, 4, 5];
    //arrays are size fixed and have only elements with the same type 
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    //can also initialize an array with the same value for each element
    let a = [3; 5]; // creates [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
}
