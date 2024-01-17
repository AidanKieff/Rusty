// Fix the evaluate method by propagating the error.

// This enum can represent any mathematical expression
enum Expression {
    Val(i32),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}
use {Expression::Add, Expression::Div, Expression::Mul, Expression::Sub, Expression::Val};
impl Expression {
    fn evaluate(&self) -> Result<i32, String> {
        match self {
            Val(val) => Ok(*val),
            Add(exp1, exp2) => {
                let val1 = exp1.evaluate();
                let val2 = exp2.evaluate();
                Ok(val1 + val2)
            }
            Sub(exp1, exp2) => {
                let val1 = exp1.evaluate();
                let val2 = exp2.evaluate();
                Ok(val1 - val2)
            }
            Mul(exp1, exp2) => {
                let val1 = exp1.evaluate();
                let val2 = exp2.evaluate();
                Ok(val1 * val2)
            }
            Div(exp1, exp2) => {
                let val1 = exp1.evaluate();
                let val2 = exp2.evaluate();
                if val2 == 0 {
                    return Err("Can not divide by zero".to_string());
                }
                Ok(val1 / val2)
            }
        }
    }
    fn to_boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

fn main() {
    // calculate: 2 + 3 * 4
    let exp = Add(
        Val(2).to_boxed(),
        Mul(Val(3).to_boxed(), Val(4).to_boxed()).to_boxed(),
    );
    match exp.evaluate() {
        Ok(res) => println!("Expression evaluated to: {res}"),
        Err(error) => println!("Error: {error}"),
    }
}
