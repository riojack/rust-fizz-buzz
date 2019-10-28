pub fn fizzbuzz(num: usize) -> String {
    if num == 3 {
        return String::from("fizz");
    }

    if num == 5 {
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
}
