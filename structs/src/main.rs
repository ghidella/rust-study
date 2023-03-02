struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple structs / useful when you want to give the whole tuple
//a name and make the tuple a different type from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs without any fields can be useful when you need
//to implement a trait on some type but don't havy any data that you
//want to store in the type itself
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("ghidella"),
        email: String::from("example@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("ghidella2"),
        email: String::from("example2@example.com"),
        sign_in_count: 0,
    };

    user2.sign_in_count = 1;

    //creating new user with another user values
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //struct update syntax
    //let user3 = User {
    //    email: String::from("otheranother@example.com"),
    //    ..user1
    //};

    //cannot use this function because ths truct update syntax uses =
    //like an assignment; this is because it moves the data

    let black = Color(0, 0, 0); //color type
    let origin = Point(0, 0, 0); // point type

    let subject = AlwaysEqual;
    
    
}

fn build_user_top(email: String, username: String) -> User { //it makes sense use the same name
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//because the name of function parameters are the same as struct fields
// we can do this without repeat email and username
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
