#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self is short for self: &self
        //self is an alias for the type that the impl block is for
        //methods must have a parameter named self of type self
        //for their first parameter spot
        self.width * self.height
    }
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
    println!("rect1 is {:#?}", rect1);

    //another way to print out a value using Debug is the dbg! macro
    //it takes the ownership of an expression(as opposed to println!)
    //prints the file and line where the call occurs and returns ownership 
    dbg!(&rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("\n######################################################\n");

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
}

//immutable borrow of struct rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}