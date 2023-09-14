#[cfg(test)]
mod tests {
    use rust_grrs::find_match;

    #[test]
    pub fn test_find_match() {
        let mut result = Vec::new();
        find_match("first line\nsecond line\nthird", "third", &mut result);
        assert_eq!(result, b"third\n");
    }
}
