fn main() {
    println!("{}", hello());
}
fn hello() -> &'static str{
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn eq() {
        assert_eq!(hello(), "Hello, world!")
    }
    #[test]
    fn ne() {
        assert_ne!(hello(), "Hi!")
    }
}
