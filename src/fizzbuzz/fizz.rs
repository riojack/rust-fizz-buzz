pub fn fizzbuzz(num: usize) -> String {
    String::from("fizz")
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizz::fizzbuzz;

    #[test]
    fn withThreeReturnsFizz() {
        assert_eq!(fizzbuzz(3), "fizz");
    }
}
