struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x (&self) -> &X1 {
        return &self.x;
    }
    
    fn mixup<X2, Y2> (self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}

impl Point<f32, f32> {
    fn distance_from_origin (&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

fn main() {
    //let number_list = vec![34, 50, 25, 100, 65];
    let number_list: [i32;5] = [34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));
    
    let integer = Point {
        x: 5,
        y: 10,
    };
    let float = Point {
        x: 1.0,
        y: 4.0,
    };
    
    let p1 = Point {
        x: 5,
        y: 10.4,
    };
    let p2 = Point {
        x: "Hello",
        y: 'c',
    };
    let p3 = p1.mixup(p2);
    println!("p3.x: {}, p3.y: {}", p3.x(), p3.y)
}

fn largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest: &T = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
