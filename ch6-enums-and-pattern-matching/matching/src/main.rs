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
    println!("{}", value_in_cents(Coin::Quarter(USState::Alaska)));
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Nickel));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),
        //_ => reroll(),
        _ => (),    //NOTE: do nothing
    }
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        }
        Coin::Nickel => {return 5;},
        Coin::Dime => return 10,
        //Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        }
    }
}

fn plus_one (x: Option<i32>) -> Option<i32> {
//fn plus_one (x: Option<i32>) -> i32 {
    match x {
        None => return None,
        Some(i) => return Some(i + 1),
        //None => return 0,
        //Some(i) => return i + 1,
    }
}

fn add_fancy_hat () {}
fn remove_fancy_hat () {}
fn move_player (num_spaces: u8) {}
fn reroll () {}
