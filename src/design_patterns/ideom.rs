// 1. use 'borrowed type' as arguments
// ex.
//   &String :- instead of -> &str
//   &Vec<T> :- instead of -> &[T]
//   &Box<T> :- instead of -> &T
pub fn three_vowels(word: &str) -> bool {
    // this function work on &String and &str
    // the case of &String is main.rs
    // the case of &str is test_three_vowels
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_three_vowels() {
        let test_case = vec![("Ferris", false), ("Curious", true)];
        for (input, expected) in test_case.into_iter() {
            assert_eq!(three_vowels(input), expected)
        }
    }
}
