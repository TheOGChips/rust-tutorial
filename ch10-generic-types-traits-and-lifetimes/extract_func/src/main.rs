fn main() {
    //let number_list = vec![34, 50, 25, 100, 65];
    let number_list: [i32;5] = [34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
    
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("The largest number is {}", largest(&number_list));
}

fn largest (list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
