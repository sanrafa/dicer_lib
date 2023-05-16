extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate anyhow;
mod dice;
mod pool_parser;
mod roll_parser;

pub use pool_parser::execute_pool as pool;
pub use roll_parser::execute_roll as roll;

#[cfg(test)]
mod tests {
    use crate::dice::*;
    use crate::pool_parser::execute_pool;
    use crate::roll_parser::execute_roll;

    #[test]
    fn dice_throws() {
        // 1d10
        let result = throw(1, 10);
        assert_eq!(result.len(), 1);
        assert_eq!(result.iter().find(|&&x| x < 1 || x > 10), None);
        // 4d123
        let result = throw(4, 123);
        assert_eq!(result.len(), 4);
        assert_eq!(result.iter().find(|&&x| x < 1 || x > 123), None);
        // 23d6
        let result = throw(23, 6);
        assert_eq!(result.len(), 23);
        assert_eq!(result.iter().find(|&&x| x < 1 || x > 6), None);
    }

    #[test]
    fn invalid_roll() {
        let result = execute_roll(".");
        assert!(result.is_err());

        let result = execute_roll("67 + 22 /");
        assert!(result.is_err());

        let result = execute_roll("(2+10/2)-((8+4)+5*)");
        assert!(result.is_err());

        let result = execute_roll("");
        assert!(result.is_err());

        let result = execute_roll("10d28+");
        assert!(result.is_err());

        let result = execute_roll("*4d6!^");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_pool() {
        let result = execute_pool(".", 10, 0.5);
        assert!(result.is_err());

        let result = execute_pool("      ", 10, 0.5);
        assert!(result.is_err());

        let result = execute_pool("2+3d6", 10, 0.5);
        assert!(result.is_err());

        let result = execute_pool("", 10, 0.5);
        assert!(result.is_err());

        let result = execute_pool("10d28+", 10, 0.5);
        assert!(result.is_err());

        let result = execute_pool("4d6!+2_3", 10, 0.5);
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
    fn pool() {
        let result = execute_pool("1+2+3", 10, 11.0).unwrap();
        assert_eq!(result.len(), 6);
        assert_eq!(
            result
                .iter()
                .find(|(x, y)| x < &&1 || x > &&10 || *y == true),
            None
        );

        let result = execute_pool("4+16-10", 6, 0.5).unwrap();
        assert_eq!(result.len(), 10);
        assert_eq!(result.iter().find(|(x, _)| x < &&1 || x > &&6), None);

        let result = execute_pool("3d6+4d10", 6, 0.0).unwrap();
        assert_eq!(result.len(), 7);
        assert_eq!(
            result
                .iter()
                .find(|(x, y)| x < &&1 || x > &&10 || *y == false),
            None
        );

        let result = execute_pool("20d2!", 6, 3.0).unwrap();
        assert!(result.len() >= 20);
        assert_eq!(
            result
                .iter()
                .find(|(x, y)| x < &&1 || x > &&2 || *y == true),
            None
        );

        let result = execute_pool("3d6-4", 6, 0.5).unwrap();
        assert!(result.len() == 1);
        assert_eq!(result.iter().next().unwrap(), &(0, false));

        let result = execute_pool("100d4-7", 6, 1.0).unwrap();
        assert!(result.len() == 93);
        assert_eq!(result.iter().find(|(x, _)| x < &&1 || x > &&4), None);
        assert!(result.iter().find(|(_, y)| *y == true).is_some());
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

    #[test]
    fn dice_rolls() {
        let result = execute_roll("1d10");
        assert!(result.is_ok());
        assert!(result.unwrap() <= 10);

        let result = execute_roll("2d6+2/3");
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() >= &1);
        assert!(result.as_ref().unwrap() <= &((2 * 6) + 2 / 3));

        let result = execute_roll("1d12*2*(3/4)");
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() >= &0);
        assert!(result.as_ref().unwrap() <= &(12 * 2 * (3 / 4)));
    }
    #[test]
    fn exploded_rolls() {
        let result = execute_roll("12d3!");
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() >= &1);
        assert!(result.as_ref().unwrap() <= &(12 * 12 * 3));

        let result = execute_roll("2*4d12!+(4/10)");
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() >= &1);
        assert!(result.as_ref().unwrap() <= &(2 * (4 * 4 * 12) + (4 / 10)));
    }
}
