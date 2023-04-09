/* NOTE: Just using "str" never works because rustc doesn't know
 *       what the size of the string is at compile time, so I guess
 *       "str" string slices must always be references.
 */

fn main() {
    /*let str1: &str = "str1";
    let str2 = print_str(str1);
    println!("main: {str2}");*/

    let mut str1: &str = "str1";
    str1 = print_str(str1);
    println!("main: {str1}");
}

fn print_str (s: &str) -> &str {
    println!("print_str: {s}");
    return "returned";
}
