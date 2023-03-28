// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    let chars_iter = input.char_indices();

    let mut i = 0;
    let mut j = input.len() - 1;

    // this clone wont clone the underlaying string but just copy the pointer.
    for (x, c) in chars_iter.clone() {
        if c != ' ' {
            i = x;
            break;
        }
    }

    for (x, c) in chars_iter.rev() {
        if c != ' ' {
            j = x;
            break;
        }
    }

    String::from(&input[i..=j])
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    
    // need to make lhs owned since it has to allocate. Addition is implemented in such a way that
    // the lhs argument is going to be used in the adition.
    input.to_owned() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
