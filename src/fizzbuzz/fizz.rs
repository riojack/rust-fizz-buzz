#[allow(dead_code)]
pub fn fizzbuzz(num: usize) -> String {
    match num {
        0 => String::from("0"),
        x if x % 3 == 0 && x % 5 == 0 => String::from("fizzbuzz"),
        x if x % 3 == 0 => String::from("fizz"),
        x if x % 5 == 0 => String::from("buzz"),
        _ => format!("{num}", num = num)
    }
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizz::fizzbuzz;

    #[test]
    fn with_three_returns_fizz() {
        assert_eq!(fizzbuzz(3), "fizz");
    }

    #[test]
    fn with_five_returns_buzz() {
        assert_eq!(fizzbuzz(5), "buzz");
    }

    #[test]
    fn with_zero_returns_zero() {
        assert_eq!(fizzbuzz(0), "0");
    }

    #[test]
    fn with_six_returns_fizz() {
        assert_eq!(fizzbuzz(6), "fizz");
    }

    #[test]
    fn with_ten_returns_buzz() {
        assert_eq!(fizzbuzz(10), "buzz");
    }

    #[test]
    fn with_fifteen_returns_fizz_buzz() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
    }
}
