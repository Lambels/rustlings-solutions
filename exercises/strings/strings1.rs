// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// lifetime annotation needed here to ensure to the compiler that the reference we are producing in
// this function isnt local to the stack of the function.
//
// We do that by saying that the str is static.
fn current_favorite_color() -> &'static str {
    "blue"
}
