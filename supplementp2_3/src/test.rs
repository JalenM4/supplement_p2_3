#[cfg(test)]

mod tests {
    use crate::add_stack;
    use crate::add_heap;
    use crate::add_mixed;
    use crate::Point3D;
    use crate::swap;
    use crate::max;

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

    #[test]
    fn test_point3d_addition() {
        let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = Point3D { x: 4.0, y: 5.0, z: 6.0 };
        let result = p1 + p2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_swap() {
        let (x, y) = swap(10, 20);
        assert_eq!(x, 20);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(15, 25), 25);
        assert_eq!(max(3.5, 2.5), 3.5);
    }

}