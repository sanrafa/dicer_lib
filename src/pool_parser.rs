use super::dice::{explode_pool, throw_pool};
use anyhow::Result;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pool.pest"]
pub struct PoolParser;

lazy_static::lazy_static! {
    static ref PRATT_POOL: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
    };
}

fn parse_pool(
    pairs: Pairs<Rule>,
    pratt: &PrattParser<Rule>,
    base: i32,
    threshold: f64,
) -> Vec<(i32, bool)> {
    pratt
        .map_primary(|p| match p.as_rule() {
            Rule::exploded => {
                let die = p.into_inner().next().unwrap();
                let mut iter = die.into_inner().take(2);
                let total = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                let faces = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                explode_pool(total, faces, threshold)
            }
            Rule::dice => {
                let mut iter = p.into_inner().take(2);
                let total = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                let faces = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                throw_pool(total, faces, threshold)
            }
            Rule::int => {
                let total = p.as_str().parse::<i32>().unwrap();
                throw_pool(total, base, threshold)
            }
            rule => unreachable!("Unexpected token: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => [lhs, rhs].concat(),
            Rule::subtract => {
                // we'll ignore the actual dice rolls and subtract dice from pool
                if lhs.len() > rhs.len() {
                    let end = lhs.len() - rhs.len();
                    lhs[..end].to_vec()
                } else {
                    // inherent failure if subtracting more dice than already in pool
                    vec![(0, false)]
                }
            }
            rule => unreachable!("Unexpected token: {:?}", rule),
        })
        .parse(pairs)
}

pub fn execute_pool(input: &str, base: i32, threshold: f64) -> Result<Vec<(i32, bool)>> {
    match PoolParser::parse(Rule::pool, input) {
        Ok(pairs) => match PoolParser::parse(Rule::expr, pairs.as_str()) {
            Ok(mut pairs) => Ok(parse_pool(
                pairs.next().unwrap().into_inner(),
                &PRATT_POOL,
                base,
                threshold,
            )),
            Err(_) => Err(anyhow!(
                "Expressions mixing dice and integers cannot start with an integer."
            )),
        },
        Err(_) => Err(anyhow!("Error parsing input.")),
    }
}
