use std::ops::Deref;
//NOTE: The DerefMut trait would be used for mutable references

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new (x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref (&self) -> &Self::Target {
        return &self.0;
    }
}

fn main() {
    let x = 5;
    //let y = &x;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{}", *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let n = MyBox::new(60);
    //let n: MyBox<i16> = MyBox::new(60); //NOTE: This doesn't work
    print_num(&n);
}

fn print_num (num: &i32) {
    println!("num: {num}");
}

fn hello (name: &str) {
    println!("Hello, {name}!");
}
