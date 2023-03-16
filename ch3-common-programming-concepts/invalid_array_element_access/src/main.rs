use std::io;

fn main() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    
    println!("Please enter an array index.");
    let mut index: String = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
        
    let element: u8 = a[index];
    println!("The value of the element at index {index} is: {element}");
    //println!("The address of a is: {a}"); //NOTE: This doesn't compile
}
