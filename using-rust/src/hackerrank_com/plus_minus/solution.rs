use crate::utils::collection::reduce_fn::*;

pub struct PlusMinus {
    n: usize,
    arr: Vec<i32>
}

impl PlusMinus {
  fn new(arr: Vec<i32>) -> PlusMinus {
    PlusMinus {
      n: arr.len(),
      arr,
    }
  }

  pub fn test() -> Result<(), &'static str> {
    let arr = vec![-4, 3, -9, 0, 4, 1];
    let plus_minus = PlusMinus::new(arr);
    PlusMinus::solve(&plus_minus.arr);
    Ok(())
  }

  fn solve(arr: &Vec<i32>) {
    let positive = my_reducer(arr, |acc, curr, _, _| {
      if curr > 0 {
        acc + 1
      } else {
        acc
      }
    }, Some(0));

    let negative = my_reducer(arr, |acc, curr, _, _| {
      if curr < 0 {
        acc + 1
      } else {
        acc
      }
    }, Some(0));

    let zero = my_reducer(arr, |acc, curr, _, _| {
      if curr == 0 {
        acc + 1
      } else {
        acc
      }
    }, Some(0));

    println!("{:.6}", positive as f64 / arr.len() as f64);
    println!("{:.6}", negative as f64 / arr.len() as f64);
    println!("{:.6}", zero as f64 / arr.len() as f64);
  }
}