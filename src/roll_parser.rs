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
            Rule::exploded => 1, //todo
            Rule::dice => 1,     //todo
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

pub fn execute_roll(input: &str) -> i32 {
    let parsed = match RollParser::parse(Rule::roll, input) {
        Ok(mut pairs) => parse_roll(pairs.next().unwrap().into_inner(), &PRATT),
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
            0
        }
    };
    parsed
}
