fn greet() -> &'static str {
    "Hello, Tim!"
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_hello() {
        assert_eq!(greet(), "Hello, Tim!");
    }
}
