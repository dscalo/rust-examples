pub fn greet(name: &str) -> String {
    let g = format!("Hey there, {}", name);
    g
}

fn private_add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*; // need this to test private methods
    #[test]
    fn internal() {
        assert_eq!(private_add(2, 7), 9);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Dan"), "Hey there, Dan");
    }
}
