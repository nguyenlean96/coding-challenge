use rand::*;
pub struct StairCase {}

impl StairCase {
  pub fn test() {
    for _ in 0..10 {
      StairCase::solution(rand::thread_rng().gen_range(6..12));
    }
  }

  fn solution(n: i32) {
    for i in 1..(n+1) {
      println!("{}{}", " ".to_string().repeat((n - i) as usize), "#".to_string().repeat(i as usize));
    }
  }
}