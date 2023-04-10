use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway (&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        return user_preference.unwrap_or_else(|| self.most_stocked());
    }

    fn most_stocked (&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            return ShirtColor::Red;
        }
        else {
            return ShirtColor::Blue;
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with perference {:?} gets {:?}",
             user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",
             user_pref2, giveaway2);

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    };

    //NOTE: The last two seem to require type annotations now
    let add_one = |x: u32| -> u32 { x + 1 };
    //let add_one = |x| { x + 1 };
    //let add_one = |x| x + 1;

    //NOTE: A compiler error gets raised if both closure calls are performed
    let example_closure = |x| x;
    //let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //let only_borrows = || println!("From closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);

    //println!("Before calling closure: {:?}", list);
    //only_borrows();
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    //NOTE: Doesn't capture anything
    //list.sort_by_key(|r| r.width);

    let mut num_sort_operations = 0;
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        num_sort_operations += 1;   //NOTE: Only captures mutable reference
        return r.width;
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
