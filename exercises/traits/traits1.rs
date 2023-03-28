// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar` for the type `String`.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

use std::ops::Add;

trait AppendBar {
    fn append_bar(self) -> Self;
}

// Blanket implement AppendBar for all type which support the adition of a string ref.
// This includes normals strings.
//
// I am using a 'a lifetime which basically states that I want this trait to be implemented on each
// T which implements add with a string lifetime of 'a. This can be monomorphised or 'a can be
// replaced by any lifetime. I could also only implement Add for Add<&'static str ...> which would
// be more restrictive.
impl<'a, T: Add<&'a str, Output = T>> AppendBar for T {
    fn append_bar(self) -> Self {
        self.add("Bar")
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
