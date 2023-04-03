enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    for i in &v {
        println!("{i}");
        //NOTE: Compile error will occur if I attempt to change v inside the loop
    }
    
    let third: i32 = v[2];                      //NOTE: I believe this also works because v[2] is an
    println!("The thrid element is {third}");   //      i32, which is on the stack
    
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    //let does_not_exist = v[100];      //NOTE: Causes a panic
    //let does_not_exist = v.get(100);  //NOTE: Just returns None
    
    /*let first = &v[0];    //NOTE: Does not compile
    v.push(6);
    println!("The first element is {first}");*/
    
    for i in &mut v {
    //for i: &mut i32 in &mut v {   //NOTE: This doesn't compile
        *i += 50;
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
