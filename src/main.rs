mod error;
mod value;

use crate::{
    error::{Error, Result},
    value::Value,
};
use std::env;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Element {
    Value(Value),
    Operator(Op),
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Element> {
        match s {
            "+" => Ok(Element::Operator(Op::Add)),
            "/" => Ok(Element::Operator(Op::Div)),
            "*" | "x" => Ok(Element::Operator(Op::Mul)),
            "-" => Ok(Element::Operator(Op::Sub)),

            s => s
                .parse()
                .map(Element::Value)
                .map_err(|_| Error::Eval("Expected operator or operand")),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Div,
    Mul,
    Sub,
}

impl Op {
    fn apply(&self, left: Value, right: Value) -> Value {
        match self {
            Op::Add => left + right,
            Op::Div => left / right,
            Op::Mul => left * right,
            Op::Sub => left - right,
        }
    }
}

struct OperandStack<T>(Vec<T>);

impl<T> OperandStack<T> {
    fn new() -> OperandStack<T> {
        OperandStack(Vec::new())
    }

    fn push(&mut self, operand: T) {
        self.0.push(operand);
    }

    fn pop(&mut self) -> Result<T> {
        self.0.pop().ok_or(Error::Eval("Operand unavailable"))
    }
}

fn main() -> Result<()> {
    let equation: Result<Vec<Element>> = env::args().skip(1).map(|x| x.parse()).collect();

    println!("{}", equation.and_then(evaluate)?);
    Ok(())
}

fn evaluate(elements: impl IntoIterator<Item = Element>) -> Result<Value> {
    let mut stack = OperandStack::new();
    for element in elements {
        match element {
            Element::Value(operand) => stack.push(operand),
            Element::Operator(operator) => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(operator.apply(left, right));
            }
        }
    }
    stack.pop()
}
