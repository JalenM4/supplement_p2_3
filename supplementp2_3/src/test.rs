#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_add_stack() {
        assert_eq!(add_stack(3, 7), 10);
    }
}