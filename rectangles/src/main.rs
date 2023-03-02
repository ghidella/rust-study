#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("rect1 is {}", rect1);
    //this do not works because the println! macro use Display formatting
    //which can show the primitive types well in the brackets but not with structs

    println!("rect1 is {:?}", rect1);
    //the specifier :? tells we want to use an output format called
    //Debug. That enable us to print our struct in a way that is useful
    //to see its value while we're debugging our code
    //we need to add the outer attribute #[derive(Debug)] before defining the struct
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

//immutable borrow of struct rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}