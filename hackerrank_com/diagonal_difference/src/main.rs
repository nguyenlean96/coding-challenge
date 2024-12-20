use rand::Rng;

pub struct DiagonalDifference {
    n: usize,
    matrix: Vec<Vec<i32>>,
}

impl DiagonalDifference {
    fn new(n: usize, matrix: Vec<Vec<i32>>) -> Self {
        Self { n, matrix }
    }

    fn test_arr_generator(n: usize) -> Vec<Vec<i32>> {
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

    pub fn test() -> Result<(), &'static str> {
        for _ in 0..10 {
            let n = rand::thread_rng().gen_range(3..4);
            let matrix = Self::test_arr_generator(n);
            let diagonal_difference = DiagonalDifference::new(n, matrix);
            let result = diagonal_difference.solve();

            // Print the maxtrix by rows
            for row in diagonal_difference.matrix.iter() {
                println!("{:?}", row);
            }
            println!("result: {:?}", result);
        }

        Ok(())
    }

    pub fn solve(&self) -> i32 {
        let mut the_first_diagonal = 0;
        let mut the_second_diagonal = 0;

        for i in 0..self.n {
            the_first_diagonal += self.matrix[i][i];
            the_second_diagonal += self.matrix[i][self.n - i - 1];
        }

        (the_first_diagonal - the_second_diagonal).abs()
    }
}

fn main() {
    DiagonalDifference::test().unwrap_or_else(|err| println!("{}", err));
}
