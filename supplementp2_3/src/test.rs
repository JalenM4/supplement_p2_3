#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_add_stack() {
        assert_eq!(add_stack(3, 7), 10);
    }

    #[test]
    fn test_add_heap() {
        assert_eq!(add_heap(Box::new(5), Box::new(8)), 13);
    }

    #[test]
    fn test_add_mixed() {
        assert_eq!(add_mixed(4, Box::new(6)), 10);
    }
}