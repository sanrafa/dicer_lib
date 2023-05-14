extern crate pest;
#[macro_use]
extern crate pest_derive;
mod roll_parser;

pub use roll_parser::execute_roll as roll;

#[cfg(test)]
mod tests {
    use crate::roll_parser::execute_roll;

    #[test]
    fn invalid_input() {
        let result = execute_roll(".");
        assert!(result.is_err());

        let result = execute_roll("67 + 22 /");
        assert!(result.is_err());

        let result = execute_roll("(2+10/2)-((8+4)+5*)");
        assert!(result.is_err());
    }

    #[test]
    fn basic_arithmetic() {
        let result = execute_roll("67 + 22").unwrap();
        assert_eq!(result, 89);

        let result = execute_roll("6-2+(3+12)").unwrap();
        assert_eq!(result, 19);

        let result = execute_roll("99 - 27").unwrap();
        assert_eq!(result, 72);

        let result = execute_roll("(50 + 30) + (80 - 25)").unwrap();
        assert_eq!(result, 135);

        let result = execute_roll("(20+10)-((8+4)+5)").unwrap();
        assert_eq!(result, 13);

        let result = execute_roll("-50 + (-30) + 10").unwrap();
        assert_eq!(result, -70);
    }

    #[test]
    fn complex_arithmetic() {
        let result = execute_roll("((50 + 20) * 3) - (8 * 10)").unwrap();
        assert_eq!(result, 130);

        let result = execute_roll("(25+15)/((6*2)-2)").unwrap();
        assert_eq!(result, 4);

        let result = execute_roll("((4*3)+5)/(9-1)").unwrap();
        assert_eq!(result, 2);

        let result = execute_roll("(12 + 9) * (35 - 23)").unwrap();
        assert_eq!(result, 252);

        let result = execute_roll("(80-25)+(50*3)/7").unwrap();
        assert_eq!(result, 76);
    }
}
