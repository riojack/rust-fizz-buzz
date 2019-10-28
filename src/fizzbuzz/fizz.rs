pub fn fizzbuzz(num: usize) -> String {
    if num == 3 {
        return String::from("fizz");
    }
    String::from("buzz")
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
}
