// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long"); // 'static
    let result;
    //{
    //    let string2 = String::from("xyz"); // gets dropped after this block, this will create a
    //                                       // lifetime as long as the scope: 'a
    //    result = longest(string1.as_str(), string2.as_str());
    //}
    //println!("The longest string is '{}'", result); // we try to use the borrowed value result
                                                    // which doesnt live long enough.
    let string2 = String::from("xyz");
    result = longest(string1.as_ref(), string2.as_ref());
    println!("The longest string is '{}'", result);
}
