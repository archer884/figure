use std::env;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Element {
    Value(i64),
    Operator(Op),
}

impl FromStr for Element {
    type Err = EvalError;

    fn from_str(s: &str) -> Result<Element, EvalError> {
        match s {
            "+" => Ok(Element::Operator(Op::Add)),
            "/" => Ok(Element::Operator(Op::Div)),
            "*" | "x" => Ok(Element::Operator(Op::Mul)),
            "-" => Ok(Element::Operator(Op::Sub)),

            s => s
                .parse()
                .map(Element::Value)
                .map_err(|_| EvalError("Expected operator or operand")),
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
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Op::Add => left + right,
            Op::Div => left / right,
            Op::Mul => left * right,
            Op::Sub => left - right,
        }
    }
}

struct OperandStack(Vec<i64>);

impl OperandStack {
    fn new() -> OperandStack {
        OperandStack(Vec::new())
    }

    fn push(&mut self, operand: i64) {
        self.0.push(operand);
    }

    fn pop(&mut self) -> Result<i64, EvalError> {
        self.0.pop().ok_or(EvalError("Operand unavailable"))
    }
}

#[derive(Debug)]
struct EvalError(&'static str);

fn main() -> Result<(), EvalError> {
    let equation: Result<Vec<Element>, _> = env::args().skip(1).map(|x| x.parse()).collect();

    println!("{}", equation.and_then(evaluate)?);
    Ok(())
}

fn evaluate(elements: impl IntoIterator<Item = Element>) -> Result<i64, EvalError> {
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
