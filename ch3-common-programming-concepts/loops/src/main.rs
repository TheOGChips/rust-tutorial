fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
            //break counter *= 2;   //NOTE: This works if you print out counter
        }
    };
    
    println!("The result is: {result}");
    //println!("The result is: {counter}");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");
        number -= 1;
        //number--; //invalid
    }
    
    println!("LIFTOFF!");
    
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    
    for number in (1..4).rev() {
    //for number in 1..4 {  //NOTE: Doesn't need parantheses if used like this
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
