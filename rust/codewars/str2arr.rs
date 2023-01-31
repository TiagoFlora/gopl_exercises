// Write a function to split a string and convert it into an array of words.
use std::str::Split;

fn string_to_array(s: &str) -> Vec<String> {
    let split_str: Vec<&str> = s.split(" ").collect();
    let result: Vec<String> = split_str.iter().map(|&x| x.to_string()).collect();
    result
}

#[cfg(test)]
mod tests {
    use super::string_to_array;

    fn dotest(s: &str, expected: &[&str]) {
        let actual = string_to_array(s);
        assert!(actual == expected, "Test failed with s = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest("Robin Singh", &["Robin", "Singh"]);
        dotest("CodeWars", &["CodeWars"]);
        dotest("I love arrays they are my favorite", &["I", "love", "arrays", "they", "are", "my", "favorite"]);
        dotest("1 2 3", &["1", "2", "3"]);
    }
}
