#[cfg(test)]
mod tests {
    use crate::megaverse::astral::objects::{AstralObject, Color, Direction};

    #[test]
    fn test_polyanet_parsing() {
        let parsed = AstralObject::build_from_string(0, 1, "POLYANET".to_string()).unwrap();
        let expected = AstralObject::Polyanet { row: 0, column: 1 };
        assert_eq!(parsed, expected);
    }
    #[test]
    fn test_cometh_parsing() {
        let parsed = AstralObject::build_from_string(0, 1, "LEFT_COMETH".to_string()).unwrap();
        let expected = AstralObject::Cometh {
            row: 0,
            column: 1,
            direction: Direction::Left,
        };
        assert_eq!(parsed, expected);
    }
    #[test]
    fn test_soloon_parsing() {
        let parsed = AstralObject::build_from_string(0, 1, "PURPLE_SOLOON".to_string()).unwrap();
        let expected = AstralObject::Soloon {
            row: 0,
            column: 1,
            color: Color::Purple,
        };
        assert_eq!(parsed, expected);
    }
}
