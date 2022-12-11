use nom::{
    IResult,
    combinator::{map, map_res},
    character::{complete::{alpha1, digit1, char, line_ending}, streaming::space1},
    bytes::complete::tag,
    sequence::{tuple, pair, separated_pair, preceded, delimited},
    branch::alt,
    multi::separated_list1, error::Error,
};
use std::str::FromStr;

//TODO try templating these functions !
pub fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Test{
    divider : i32,
    monkey_false: usize,
    monkey_true: usize,   
}

impl Test{
    fn parse(input: &str) -> IResult<&str, Self>{
        let divider = delimited(pair(space1,tag("Test: divisible by ")), parse_number, line_ending);
        let monkey_true = delimited(pair(space1,tag("If true: throw to monkey ")), parse_usize, line_ending);
        let monkey_false = delimited(pair(space1,tag("If false: throw to monkey ")), parse_usize, line_ending);
        map(tuple((divider,monkey_true,monkey_false)),
         |(div, mt, mf)| Test{divider: div, monkey_false: mf, monkey_true:mt})(input)
    }
}

enum Operator{
    Mult,
    Add,
}
enum Operand{
    Old,
    Nb(i32),
}
struct Operation{
    operator: Operator,
    operand: Operand,
}

impl Operation{
    fn parse(input: &str) -> IResult<&str, Self>{
        let operator = map_res(alpha1, |i| {
            match i {
                 "*" => Operator::Mult,
                 "+" => Operator::Add,
                 _ => Error("not a valid operator")
                }
            });
        let computation = 
        let parser = delimited(pair(space1,tag("Operation: new = old "), computation, line_ending);
    }
}
struct Monkey{
    items : Vec<i32>,
    op : Operation,
    test : Test,
}

impl Monkey{
    fn parse(input: &str) -> IResult<&str, Self>{
        let headline = tuple((space1, tag("Monkey "), digit1, tag(":"), line_ending));
        let items = delimited(pair(space1, tag("Starting items: ")),separated_list1(tag(", "), parse_number),line_ending);
        map(preceded(headline, tuple((items,Operation::parse,Test::parse))), |(i,o,t)| Monkey{items:i, op: o, test:t})(input)
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test()
    {
        let input ="    Test: divisible by 19
        If true: throw to monkey 4
        If false: throw to monkey 7\n";

        assert_eq!(Test::parse(input).unwrap().1, Test{divider:19, monkey_true: 4, monkey_false: 7});
    }
}