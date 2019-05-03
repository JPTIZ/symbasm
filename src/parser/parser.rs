// External libraries
extern crate pest;

use pest::{iterators::Pair, Parser};

// Parser data
#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct SymblyParser;


#[derive(Debug, Clone)]
pub struct ParseError;

fn process_operand(pair: Pair<Rule>) -> Result<&str, &str> {
    match pair.as_rule() {
        Rule::register => {
            println!("    [Register Operand] {}", pair.as_str());

            return Ok(pair.as_str());
        },
        Rule::number => {
            println!("    [Literal Operand] {}", pair.as_str());

            return Ok(pair.as_str());
        },
        _ => Err("Invalid Operand"),
    }
}

fn process(pair: Pair<Rule>) {
    match pair.as_rule() {
        Rule::instruction => {
            println!("Found instruction: {}", pair.as_str());

            let mut pairs = pair.into_inner();

            process(pairs.next().unwrap());
        },
        Rule::add_instruction => {
            println!("Found add instruction: {}", pair.as_str());

            let mut pairs = pair.into_inner();

            let (rd, r1, r2) = (
                process_operand(pairs.next().unwrap()),
                process_operand(pairs.next().unwrap()),
                process_operand(pairs.next().unwrap()),
            );

            println!(
                "Sum {:?} + {:?} and store into {:?}",
                 r1.unwrap(),
                 r2.unwrap(),
                 rd.unwrap(),
            );
        },
        Rule::register => {
            println!("Found register {}", pair.as_str());
        },
        Rule::number => {
            println!("Found number literal: {}", pair.as_str());
        },
        Rule::program => {
            println!("Found program start: {}", pair.as_str());
            let mut pairs = pair.into_inner();

            process(pairs.next().unwrap());
        },
        _ => {
            println!("{:?} -> {}", pair.as_rule(), pair.as_str());
        }
    }
}

// &str is just a placeholder type for now.
pub fn parse(text: &str) -> Result<&str, ParseError> {
    println!("Parsing \"{}\"", text);

    match SymblyParser::parse(Rule::program, text) {
        Ok(mut pairs) => {
            process(pairs.next().unwrap());
        }
        Err(error) => {
            println!("FAILED!\n{}", error);
            return Err(ParseError{});
        }
    }

    // TODO!
    Ok("OK")
}
