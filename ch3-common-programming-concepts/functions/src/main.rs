fn main() {
    print_labeled_measurement(5, 'h');
    //print_labeled_measurement("10mm");
    println!("five(): {}", five());
}

fn print_labeled_measurement (value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//NOTE: Function overloading is not allowed. This will cause an error.
/*
fn print_labeled_measurement (value: String) {
    println!("The measurement is: {value}");
}
*/

fn five () -> i32 {
    //5         //valid
    //5;        //invalid
    //return 5  //valid
    return 5;   //valid
}
