// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // 1. Move the string2 declaration to make it live as long as string1 (how is result declared?)
    let string2 = String::from("xyz");
    let result;
    {
        // let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // 2. Move println! into the inner block
        // println!("The longest string is '{}'", result);
    }
    println!("The longest string is '{}'", result);
}
