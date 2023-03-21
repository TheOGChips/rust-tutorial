#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        return self.width * self.height;
    }
    
    fn can_hold (&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
    
    fn square (size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        //width: dbg!(30 * 2),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("rect1 is: {:?}", rect1);
    println!("rect1 is: {:#?}", rect1);
    //dbg!(&rect1);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    println!("A square: {:?}", Rectangle::square(25));
    println!("A square: {:#?}", Rectangle::square(35));
}


