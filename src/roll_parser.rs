use super::dice::{explode, roll};
use pest::error::Error;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "roll_parser/roll.pest"]
pub struct RollParser;

lazy_static::lazy_static! {
    static ref PRATT: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(neg))
    };
}

fn parse_roll(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> i32 {
    pratt
        .map_primary(|p| match p.as_rule() {
            Rule::exploded => {
                let dice_rule = p.into_inner().next();
                if let Some(die) = dice_rule {
                    let mut iter = die.into_inner().take(2);
                    let total = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                    let faces = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                    return explode(total, faces);
                }
                0
            }
            Rule::dice => {
                let mut iter = p.into_inner().take(2);
                let total = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                let faces = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                roll(total, faces)
            }
            Rule::int => p.as_str().parse::<i32>().unwrap(),
            Rule::expr => parse_roll(p.into_inner(), &PRATT),
            rule => unreachable!("Unexpected token: {:?}", rule),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => -rhs,
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn execute_roll(input: &str) -> Result<i32, Error<Rule>> {
    match RollParser::parse(Rule::roll, input) {
        Ok(mut pairs) => Ok(parse_roll(pairs.next().unwrap().into_inner(), &PRATT)),
        Err(e) => Err(e),
    }
}
