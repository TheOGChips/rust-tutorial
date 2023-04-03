use std::fs::{self,
              File};
use std::io::{self,
              ErrorKind,
              Read};
use std::error::Error;

fn main_other() {
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");
    /*let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        //Err(error) => panic!("Problem opening the file: {:?}", error), //NOTE: Doing something other
                                                                       //      than this gives an
                                                                       //      error and is more
                                                                       //      trouble than it's
                                                                       //      worth.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };*/
    
    //let greeting_file: File = File::open("hello.txt").unwrap();
    let greeting_file: File = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn main () -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;   //NOTE: Returns nonzero on error
    return Ok(());  //NOTE: Returns 0 on success
}

fn read_username_from_file() -> Result<String, io::Error> {
    //NOTE: Method 1
    /*let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    return match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };*/
    
    //NOTE: Method 2
    /*
    //let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    //username_file.read_to_string(&mut username)?;
    File::open("hello.txt")?.read_to_string(&mut username)?;
    return Ok(username);
    */
    
    //NOTE: Method 3
    return fs::read_to_string("hello.txt");
}

fn last_char_of_first_line (text: &str) -> Option<char> {
    return Some(text.lines().next()?.chars().last()?);
}
