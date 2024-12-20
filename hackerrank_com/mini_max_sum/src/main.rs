use utils::collection::{generator::array_generator, reduce_fn::*, sort::merge_sort::merge_sort};

pub struct MiniMaxSum {
    pub arr: Vec<i64>,
}

impl Default for MiniMaxSum {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl MiniMaxSum {
    fn new(arr: Vec<i64>) -> Self {
        Self { arr }
    }

    pub fn test() -> Result<(), &'static str> {
        let mut arr = array_generator(5);

        merge_sort(&mut arr);

        let min_sum = my_reducer(&arr[..4].to_vec(), |acc, x, _, _| acc + x, Some(0));

        let max_sum = my_reducer(&arr[1..].to_vec(), |acc, x, _, _| acc + x, Some(0));

        println!("min_sum: {:?}", min_sum);
        println!("max_sum: {:?}", max_sum);
        Ok(())
    }
}

fn main() {
    MiniMaxSum::test().unwrap_or_else(|err| println!("{}", err));
}
