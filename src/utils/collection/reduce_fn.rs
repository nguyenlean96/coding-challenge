pub fn my_reducer(arr: &Vec<i32>, callback: fn(i32, i32, Option<i32>, Option<&Vec<i32>>) -> i32, initial_value: Option<i32>) -> i32 {
  let mut accumulator = 0;
  let mut start_index = 0;

  if let Some(initial_value) = initial_value {
      accumulator += initial_value;
  } else {
      start_index = 1;
  }

  for i in start_index..arr.len() {
      accumulator = callback(accumulator, arr[i], Some(i as i32), Some(&arr));
  }

  return accumulator;
}