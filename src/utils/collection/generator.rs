use rand::*;

pub fn array_generator(n: usize) -> Vec<i32> {
  let mut arr = vec![];
  for _ in 0..n {
      arr.push(rand::thread_rng().gen_range(-9..9));
  }
  arr.clone()
}

pub fn two_dimension_matrix_generator(n: usize) -> Vec<Vec<i32>> {
  let mut matrix = vec![];
  for _ in 0..n {
      let mut row = vec![];
      for _ in 0..n {
          row.push(rand::thread_rng().gen_range(-9..9));
      }
      matrix.push(row);
  }
  matrix
}