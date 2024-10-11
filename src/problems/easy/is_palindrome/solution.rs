use rand::Rng;
pub struct IsPalindrome {}

impl IsPalindrome {
    pub fn test() -> Result<(), String> {
        let x = rand::thread_rng().gen_range(0..9999);
        let result = IsPalindrome::is_palindrome(x);
        println!("result: {:?}", result);
        let correct = result == true;
        if correct {
            Ok(())
        } else {
            Err("The result is incorrect.".to_string())
        }
    }

    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        println!("x: {:?}", x);
        let mut x = x;
        let mut reversed = 0;
        
        while x > 0 {
            reversed = reversed * 10 + x % 10;
            x /= 10;

            println!("reversed: {:?}, x: {:?}", reversed, x);
        }
        
        reversed == x
    }
}