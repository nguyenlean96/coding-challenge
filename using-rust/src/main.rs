// Use the problems from lib.rs
use mycrate::problems::{
    easy::{
        is_palindrome::solution::IsPalindrome, 
        two_sum::solution::TwoSum
    },
    hard::text_justification::solution::*,
};
use mycrate::hackerrank_com::{
    diagonal_difference::solution::*,
    plus_minus::solution::*,
    stair_case::solution::*,
    mini_max_sum::solution::*,
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

    DiagonalDifference::test().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });

    PlusMinus::test().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });

    StairCase::test();

    MiniMaxSum::test().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}
