#[cfg(feature = "one")]
/// Does a thing.
pub fn blah() {
    println!("blah");
}

#[cfg(feature = "two")]
/// Does a different thing.
pub fn bork() {
    println!("bork");
}

/// Always there for you.
pub fn yo() {
    println!("yo!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
