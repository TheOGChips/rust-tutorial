#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Tennessee,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main() {
    //println!("Hello, world!");
    let config_max = Some(3u8);
    /*match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }*/
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
    let mut count = 0;
    //let coin = Coin::Penny;
    let coin = Coin::Quarter(USState::Tennessee);
    
    /*
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    */
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
    else {
        count += 1;
    }
    println!("count: {count}");
}
