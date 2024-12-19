use std::collections::HashMap;

use rand::seq::{index, SliceRandom};

pub struct TwoSum {}

impl TwoSum {
    pub fn test() -> Result<(), String> {
        use rand::Rng;
        // Randomly generate a vector of numbers and all numbers are unique
        let mut nums = (0..rand::thread_rng().gen_range(3..10)).collect::<Vec<i32>>();
        nums.shuffle(&mut rand::thread_rng());
        
        // Randomly generate a target greater than 0
        let target = rand::thread_rng().gen_range(1..10);

        // Print the generated numbers and target
        println!("nums: {:?}", nums);
        println!("target: {:?}", target);

        // Call the two_sum function
        let result = TwoSum::two_sum(&nums, &target);

        // Print the result
        println!("result: {:?}", result);

        // Check if the result is correct
        let correct = result[0] + result[1] == target;

        // If the result is correct, return Ok(())
        if correct {
            Ok(())
        } else {
            // If the result is incorrect, return an error message
            Err("The result is incorrect.".to_string())
        }
    }

    fn two_sum(nums: &Vec<i32>, target: &i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32];
            }
            
            map.insert(num, i as i32);
        }
        vec![]
    }
}
