use rand::{seq::SliceRandom, Rng};

pub struct TextJustification {}

impl TextJustification {
    fn test_strings() -> Vec<Vec<String>> {
        let example_1: Vec<String> = Vec::from([
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ]);
        let example_2: Vec<String> = vec![
            "What".to_string(),
            "must".to_string(),
            "be".to_string(),
            "acknowledgment".to_string(),
            "shall".to_string(),
            "be".to_string(),
        ];
        let example_3: Vec<String> = vec![
            "Science".to_string(),
            "is".to_string(),
            "what".to_string(),
            "we".to_string(),
            "understand".to_string(),
            "well".to_string(),
            "enough".to_string(),
            "to".to_string(),
            "explain".to_string(),
            "to".to_string(),
            "a".to_string(),
            "computer.".to_string(),
            "Art".to_string(),
            "is".to_string(),
            "everything".to_string(),
            "else".to_string(),
            "we".to_string(),
            "do".to_string(),
        ];

        // Randomly return one of the examples
        let mut examples = vec![example_1, example_2, example_3];
        examples.shuffle(&mut rand::thread_rng());
        
        examples
    }

    pub fn test() -> Result<(), String> {
        let examples = TextJustification::test_strings();
        for words in examples.iter() {
            let max_width = rand::thread_rng().gen_range(18..36);
            let result = TextJustification::full_justify(words, max_width);
            println!("result: {:?}", result);
        }

        Ok(())
    }

    fn full_justify(words: &Vec<String>, max_width: i32) -> Vec<String> {
        // println!("words: {:?}, max_width: {:?}", words, max_width);
        let mut result: Vec<String> = Vec::new();
        let mut line = String::new().to_owned();

        for word in words.iter() {
            if line.len() + word.len() > max_width as usize {
                result.push(line.to_string());
                line.clear();
            }

            line.push_str(word);
            line.push_str(" ");
        }

        result
    }
}
