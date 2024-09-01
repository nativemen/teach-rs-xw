/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - We have added the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
///
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?
///
/// - EXTRA: Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Can you change the code so that that errors are propagated correctly? (hint: use the ? syntax).

#[derive(PartialEq, Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Var,
    Summation(Vec<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sigma(Box<Expr>, Box<Expr>),
}

// inject these two identifiers directly into the current namespace
use Expr::*;

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    Mul(Box::new(x), Box::new(y))
}

fn div(x: Expr, y: Expr) -> Expr {
    Div(Box::new(x), Box::new(y))
}

// ...

fn eval(expr: &Expr, var: i64) -> Option<i64> {
    match expr {
        Const(k) => Some(*k),
        Var => Some(var),
        Add(lhs, rhs) => eval(lhs, var).and_then(|l| eval(rhs, var).and_then(|r| l.checked_add(r))),
        Sub(lhs, rhs) => eval(lhs, var).and_then(|l| eval(rhs, var).and_then(|r| l.checked_sub(r))),
        Mul(lhs, rhs) => eval(lhs, var).and_then(|l| eval(rhs, var).and_then(|r| l.checked_mul(r))),
        Div(lhs, rhs) => eval(lhs, var).and_then(|l| eval(rhs, var).and_then(|r| l.checked_div(r))),
        Summation(exprs) => {
            let mut acc: i64 = 0;
            for e in exprs {
                acc = acc.checked_add(eval(e, var)?)?;
            }
            Some(acc)
        }
        Sigma(from, to) => {
            let from_val = eval(from, var)?;
            let to_val = eval(to, var)?;
            let mut exprs = Vec::new();
            let mut val = from_val;
            while val <= to_val {
                exprs.push(Const(val));
                val = val.checked_add(1)?;
            }
            eval(&Summation(exprs), var)
        }
    }
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        println!(
            "{:?} with Var = {} ==> {:?}",
            &expr,
            value,
            eval(&expr, value)
        );
    };

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(sub(Var, Var));
    test(add(sub(Var, Const(5)), Const(5)));
    test(mul(Var, Const(12)));
    test(div(Var, Const(0)));
    test(div(Var, Const(10)));
    test(Summation(vec![Var, Const(1)]));
    test(Sigma(Box::new(Const(1)), Box::new(Const(5))));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let x = 42;
        assert_eq!(eval(&Const(5), x), Some(5));
        assert_eq!(eval(&Var, x), Some(42));
        assert_eq!(eval(&sub(Var, Const(5)), x), Some(37));
        assert_eq!(eval(&sub(Var, Var), x), Some(0));
        assert_eq!(eval(&add(sub(Var, Const(5)), Const(5)), x), Some(42));
        assert_eq!(eval(&mul(Var, Const(10)), x), Some(420));
        assert_eq!(eval(&div(Var, Const(0)), x), None);
        assert_eq!(eval(&div(Const(84), Var), x), Some(2));
        assert_eq!(eval(&Summation(vec![Var, Const(1)]), x), Some(43));
        assert_eq!(
            eval(&Sigma(Box::new(Const(1)), Box::new(Const(5))), x),
            Some(15)
        );
    }
}

// If you have time left and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
