pub fn say_hello(name: &str) -> String {
    // result : create string as bellow
    // let mut result = "Hello ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result

    // but, you can use format!
    // this is easy, but it is not optimal.
    format!("Hello: {name}!", name=name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_say_hello() {
        let test_case = vec![("bob", "Hello: bob!"), ("aris", "Hello: aris!")];
        for (input, expected) in test_case.into_iter() {
            assert_eq!(say_hello(input), expected)
        }
    }
}
