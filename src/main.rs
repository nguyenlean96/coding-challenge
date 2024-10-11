// Use the problems from lib.rs
use mycrate::problems::{
    easy::{
        is_palindrome::solution::IsPalindrome, 
        two_sum::solution::TwoSum
    },
    hard::text_justification::solution::*,
};

fn main() {
    TwoSum::test().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });

    IsPalindrome::test().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });

    TextJustification::test().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}
