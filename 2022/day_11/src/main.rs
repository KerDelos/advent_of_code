use nom::{
    IResult,
    combinator::{map, map_res},
    character::{complete::{alpha1, digit1, char, line_ending, anychar, multispace1}, streaming::space1},
    bytes::complete::tag,
    sequence::{tuple, pair, separated_pair, preceded, delimited},
    branch::alt,
    multi::separated_list1, error::Error,
};
use std::{str::FromStr, cell::RefCell, borrow::Borrow};

//TODO try templating these functions !
pub fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, i64::from_str)(input)
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Test{
    divider : i64,
    monkey_true: usize,   
    monkey_false: usize,
}

impl Test{
    fn parse(input: &str) -> IResult<&str, Self>{
        let divider = delimited(pair(space1,tag("Test: divisible by ")), parse_number, line_ending);
        let monkey_true = delimited(pair(space1,tag("If true: throw to monkey ")), parse_usize, line_ending);
        let monkey_false = delimited(pair(space1,tag("If false: throw to monkey ")), parse_usize, line_ending);
        map(tuple((divider,monkey_true,monkey_false)),
         |(div, mt, mf)| Test{divider: div, monkey_false: mf, monkey_true:mt})(input)
    }
    fn compute(&self, input: i64) -> usize{
        if input % self.divider == 0 {
            return self.monkey_true;
        }
        return self.monkey_false;
    }
}
#[derive(Debug, Eq, PartialEq)]
enum Operator{
    Mult,
    Add,
}

#[derive(Debug, Eq, PartialEq)]
enum Operand{
    Old,
    Nb(i64),
}
#[derive(Debug, Eq, PartialEq)]
struct Operation{
    operator: Operator,
    operand: Operand,
}

impl Operation{
    fn parse(input: &str) -> IResult<&str, Self>{
        let operator = map_res(anychar, |i| {
            match i {
                 '*' => Ok(Operator::Mult),
                 '+' => Ok(Operator::Add),
                 _ => Err("not a valid operator"),
                }
            });
        let old_operand = map_res(tag("old"), |_| Ok::<Operand,nom::Err<i32>>(Operand::Old)); //TODO i32 should probably not be the generic type for Err
        let number_operand = map_res(parse_number, |i| Ok::<Operand,nom::Err<i32>>(Operand::Nb(i)));
        let operand = alt((old_operand, number_operand));
        let computation = separated_pair(operator, space1, operand);
        let parser = delimited(pair(space1,tag("Operation: new = old ")), computation, line_ending);
        map(parser, |(otor, oand)| Operation{operator: otor, operand: oand })(input)
    }

    fn compute(&self, input: i64) -> i64{
        let other = match self.operand {
            Operand::Old => input,
            Operand::Nb(n) => n,
        };
        match self.operator {
            Operator::Mult => input * other,
            Operator::Add => input + other,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey{
    items : Vec<i64>,
    op : Operation,
    test : Test,
}

impl Monkey{
    fn parse(input: &str) -> IResult<&str, Self>{
        let headline = tuple((tag("Monkey "), digit1, tag(":"), line_ending));
        let items = delimited(pair(space1, tag("Starting items: ")),separated_list1(tag(", "), parse_number),line_ending);
        map(preceded(headline, tuple((items,Operation::parse,Test::parse))), |(i,o,t)| Monkey{items:i, op: o, test:t})(input)
    }
}


fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").unwrap();

    let mut monkeys :Vec<RefCell<Monkey>> = Vec::new();
    for m in separated_list1(multispace1, Monkey::parse)(&content).unwrap().1
    {
        monkeys.push(RefCell::new(m));
    }

    let mut monkey_examinations : Vec<i32> = vec![0; monkeys.len()];
    
    for i in 0..20{
        println!("Round {}", i+1);
        for (i,m) in monkeys.iter().enumerate(){
            for it in &m.borrow().items{
                monkey_examinations[i] += 1;
                let new_worry_level = m.borrow().op.compute(*it) / 3;
                let new_monkey = m.borrow().test.compute(new_worry_level);
                monkeys[new_monkey].borrow_mut().items.push(new_worry_level);
                //println!("Monkey {} examined {} and passes {} to {} who now has {:?}",i,*it, new_worry_level, new_monkey, monkeys[new_monkey].borrow().items);
            }
            m.borrow_mut().items.clear();
        }
        println!("After round {} --------------------------", i+1);
        for (i,m) in monkeys.iter().enumerate(){
            println!("Monkey {} is holding {:?}", i, m.borrow().items);
        }
    }

    monkey_examinations.sort();
    monkey_examinations.reverse();
    println!("Monkey business level is {}", monkey_examinations[0..2].iter().product::<i32>())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test()
    {

        let input_items = "  Starting items: 54, 65, 75, 74\n";
        let input_op = "  Operation: new = old + 6\n";
        let input_test ="    Test: divisible by 19
        If true: throw to monkey 4
        If false: throw to monkey 7\n";

        let monkey_input = "Monkey 1:\n".to_owned() + input_items + input_op + input_test;

        let op_output = Operation{operator: Operator::Add, operand: Operand::Nb(6)};
        assert_eq!(Operation::parse(&input_op).unwrap().1, op_output);

        let test_output = Test{divider:19, monkey_true: 4, monkey_false: 7};
        assert_eq!(Test::parse(&input_test).unwrap().1, test_output);

        let monkey_output = Monkey{ items: vec![54,65,75,74], op: op_output, test: test_output};
        assert_eq!(Monkey::parse(&monkey_input).unwrap().1, monkey_output);
    }
}