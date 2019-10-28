pub fn fizzbuzz(num: usize) -> String {
    if num != 0 && num % 3 == 0 {
        return String::from("fizz");
    }

    if num != 0 && num % 5 == 0 {
        return String::from("buzz");
    }

    format!("{num}", num = num)
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizz::fizzbuzz;

    #[test]
    fn withThreeReturnsFizz() {
        assert_eq!(fizzbuzz(3), "fizz");
    }

    #[test]
    fn withFiveReturnsBuzz() {
        assert_eq!(fizzbuzz(5), "buzz");
    }

    #[test]
    fn withZeroReturnsZero() {
        assert_eq!(fizzbuzz(0), "0");
    }

    #[test]
    fn withSixReturnsFizz() {
        assert_eq!(fizzbuzz(6), "fizz");
    }

    #[test]
    fn withTenReturnsBuzz() {
        assert_eq!(fizzbuzz(10), "buzz");
    }
}
