struct User {           // NOTE: A normal struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color (i32, i32, i32);   //NOTE: A tuple struct
struct Point (i32, i32, i32);

struct AlwaysEqual;     //NOTE: A unit-like struct

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1     //NOTE: This invalidates user1.username
    };
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let subject = AlwaysEqual;
    println!("username: {}", user2.username);
}

fn build_user (email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
}
