#[cfg(test)]
mod tests {
    use crate::megaverse::utils::geo::compute_cross_coordinates;

    #[test]
    fn test_square_cross_i32() {
        let mut obtained: Vec<(i32, i32)> =
            compute_cross_coordinates(5i32, 5i32).unwrap().collect();
        let mut expected = vec![(2, 2)];
        obtained.sort();
        expected.sort();
        assert_eq!(obtained, expected)
    }
    #[test]
    fn test_not_square_cross_u32() {
        let mut obtained: Vec<(u32, u32)> =
            compute_cross_coordinates(5u32, 10u32).unwrap().collect();
        let mut expected = vec![(2, 2)];
        obtained.sort();
        expected.sort();
        assert_eq!(obtained, expected)
    }
    #[test]
    fn test_not_square_cross_u128() {
        let mut obtained: Vec<(u128, u128)> =
            compute_cross_coordinates(10u128, 10u128).unwrap().collect();
        let mut expected = vec![
            (2, 2),
            (2, 7),
            (3, 3),
            (3, 6),
            (4, 4),
            (4, 5),
            (5, 4),
            (5, 5),
            (6, 3),
            (6, 6),
            (7, 2),
            (7, 7),
        ];
        obtained.sort();
        expected.sort();
        assert_eq!(obtained, expected)
    }
    #[test]
    fn test_negative_coordinates() {
        let obtained = compute_cross_coordinates(5i32, -10i32);
        assert!(obtained.is_err());
        assert_eq!(
            obtained.err().unwrap().to_string(),
            "Geometric figure impossible to construct".to_string()
        );
    }
}
