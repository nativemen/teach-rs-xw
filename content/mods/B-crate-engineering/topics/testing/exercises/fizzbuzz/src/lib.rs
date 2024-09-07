/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> &'static str {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => "FizzBuzz",
        (true, false) => "Fizz",
        (false, true) => "Buzz",
        (false, false) => Box::leak(i.to_string().into_boxed_str()),
    }
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.

#[cfg(test)]

mod tests {
    use crate::fizz_buzz;

    #[test]
    fn test_fizzbuzz() {
        let mut index = 1;
        for line in include_str!("../fizzbuzz.out").lines() {
            assert_eq!(fizz_buzz(index), line.trim());
            index += 1;
        }
    }
}
