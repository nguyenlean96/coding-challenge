// Use the problems from lib.rs
use solve_leetcode_problems_wt_rust::problems::{
    easy::{
        is_palindrome::solution::IsPalindrome, 
        two_sum::solution::TwoSum
    },
    hard::text_justification::solution::*,
};
use solve_leetcode_problems_wt_rust::hackerrank_com::{
    diagonal_difference::solution::*,
    plus_minus::solution::*,
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
}
