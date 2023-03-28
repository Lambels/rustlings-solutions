// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.

use std::borrow::Cow;

// use 2 lifetimes to allow the compiler to monomorphise or replace them independently. This is
// because &mut T is invariant over T and borrowing the Cow smart pointer in the following way wont
// work:
//
// let mut v = ... (Cow)
// abs_all(&mut v);
// abs_all(&mut v);
// println!("{:?}", v);
//
// since v implicitly has to live till the last call to println, so if we were to use one single
// lifetime we would be forced to draw the following conections:
// abs_all<'a>(input: &'a mut Cow<'a, ...>) -> &'a mut Cow<'a, ...>
//
// now you can see that the mutable reference will live for as long as the Cow<'a, ...> smart
// pointer lives. This is problamatic because later if we want to use v, like in the second abs_all
// call we would run in a conflict of 2 mutable references being passed out which is rightfully
// caught by the borrow checker.
//
// This wouldnt be a problem if we were to use immutable references because &T is covariant in T
// and the compiler would be able to shorten the lifetime of the inner borrow automatically to make
// our code compile. This isnt the case with mutable references because the compiler cant shorten
// the lifetime or replace T with a subtype because &mut T is invariant over T.
//
// This is fixed by using 2 different lifetimes which get monomorphised separately.
fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn main() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Borrowed(_) => println!("I borrowed the slice!"),
        _ => panic!("expected borrowed value"),
    }

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I modified the slice and now own it!"),
        _ => panic!("expected owned value"),
    }

    // No clone occurs because `input` is already owned.
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    match abs_all(&mut input) {
        // TODO
        Cow::Owned(_) => println!("I own this slice!"),
        _ => panic!("expected borrowed value"),
    }
}
