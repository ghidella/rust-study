fn main() {
    // The borrowing concept is really simple, it is basic a reference like in C
    // where you can borrow a variable ownership to another scope 
    
    let mut s = String::from("Hello");

    change(&mut s);
    println!("{s}");

    // Mutable references have one big restriction
    // Do not attempt to create two mutable references to the same variable like:
    
    let r1 = &mut s;
    //let r2 = &mut s;

    // This rust behavior prevents data race

    // Still, if you need to use 2 references you can create another scope
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // However, mutiple immutable references are allowed bc no one who is just reading the data 
    // can affect anyone else's reading of the data
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
   
    // A reference's scope starts from where it is introduced and continues through the last time
    // that it is used

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
