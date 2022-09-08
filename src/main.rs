use std::{convert::TryFrom, error::Error, fmt, iter::Peekable, slice::Iter};

/// Define the tokens that the input string can have.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
    Plus,        // a => +
    Dash,        // b => -
    Star,        // c => *
    Slash,       // d => /
    LeftParen,   // e => (
    RightParen,  // f => )
    Number(i64), // regular number
    End,         // end of the expression
}

/// Define the arithmetic operations that can be performed.
#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Negative,
}

/// Define the conversion from tokens to operators
/// by implementing the TryFrom trait.
impl TryFrom<Token> for Operator {
    type Error = &'static str;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(Operator::Add),
            Token::Dash => Ok(Operator::Subtract),
            Token::Star => Ok(Operator::Multiply),
            Token::Slash => Ok(Operator::Divide),
            _ => Err("Wrong operator"),
        }
    }
}

/// Define the expressions that you can find.
/// There are three main expressions:
///     - A number (like 7)
///     - A unary operation (like -7, which is -1 * 7)
///     - A binary operation (like 3 * 4)
///
/// With these expressions you can define the abstract
/// syntax tree.
#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Number(i64),
    Unary(Operator, Box<Expression>),
    Binary(Operator, Box<Expression>, Box<Expression>),
}

/// Evaluate the expressions that you find based on the
/// type of operation that [the expression] defines.
/// The actual symbol that represents an operator will be
/// defined in the `lexicon` function.
impl Expression {
    fn eval(&mut self) -> i64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Unary(_negative, expr) => -1 * expr.eval(),
            Expression::Binary(Operator::Add, expr1, expr2) => expr1.eval() + expr2.eval(),
            Expression::Binary(Operator::Subtract, expr1, expr2) => expr1.eval() - expr2.eval(),
            Expression::Binary(Operator::Multiply, expr1, expr2) => expr1.eval() * expr2.eval(),
            Expression::Binary(Operator::Divide, expr1, expr2) => expr1.eval() / expr2.eval(),
            _ => panic!("Unreachable code for expr {:?}", self),
        }
    }
}

// First I thought about creating the syntax error with an enum,
// but they don't give back much information about the error and
// where they happened. So I decided to implement them as structs.
// #[derive(Debug, PartialEq)]
// enum SyntaxError {
//     Lexicon,
//     Parser,
// }

/// Define a structure for the syntactic errors that
/// can be found when parsing an input.
#[derive(Debug)]
struct SyntaxError {
    message: String,
    level: String,
}
/// Define how to deal with the possible syntactic errors.
impl SyntaxError {
    /// Error in the lexicon for when a symbol cannot be found.
    fn new_lex_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Lexicon".to_string(),
        }
    }
    /// Error while parsing the input.
    fn new_parse_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Parse".to_string(),
        }
    }
}

/// Pretty printing the errors.
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

/// And the error trait.
impl Error for SyntaxError {}

/// The structure of the parser to interpret the input.
/// It consists on a Peekable type, so it's possible to
/// peek the next element in the iterator without consuming it.
struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

/// Define a top-down implementation of a `recursive descent parser`.
impl<'a> Parser<'a> {
    fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        Parser { iter }
    }

    /// Assert if there is a problem with the next value in the iterator.
    fn assert_next(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();
        if let None = next {
            return Err(SyntaxError::new_parse_error(
                "End of input unexpected".to_string(),
            ));
        }

        if *next.unwrap() != token {
            return Err(SyntaxError::new_parse_error(format!(
                "Expected {:?} but actual {:?}",
                token,
                next.unwrap(),
            )));
        }

        Ok(())
    }

    /// Evaluate the expression.
    fn expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut expr: Expression = self.primary_expression()?;

        loop {
            let next = self.iter.peek().unwrap();
            match next {
                Token::Plus => {
                    self.iter.next();
                    let rhs = self.primary_expression()?;
                    expr = Expression::Binary(Operator::Add, Box::new(expr), Box::new(rhs));
                }
                Token::Dash => {
                    self.iter.next();
                    let rhs = self.primary_expression()?;
                    expr = Expression::Binary(Operator::Subtract, Box::new(expr), Box::new(rhs));
                }
                Token::Star => {
                    self.iter.next();
                    let rhs = self.primary_expression()?;
                    expr = Expression::Binary(Operator::Multiply, Box::new(expr), Box::new(rhs));
                }
                Token::Slash => {
                    self.iter.next();
                    let rhs = self.primary_expression()?;
                    expr = Expression::Binary(Operator::Divide, Box::new(expr), Box::new(rhs));
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    /// Evaluate numbers, parenthesis and minus signs `-`, and discard
    /// not known tokens.
    fn primary_expression(&mut self) -> Result<Expression, SyntaxError> {
        let next = self.iter.next().unwrap();

        match next {
            Token::Number(n) => Ok(Expression::Number(*n)),
            Token::LeftParen => {
                let expr = self.expression()?;
                self.assert_next(Token::RightParen)?;
                Ok(expr)
            }
            Token::Dash => {
                let expr = self.primary_expression()?;
                Ok(Expression::Unary(Operator::Negative, Box::new(expr)))
            }
            _ => Err(SyntaxError::new_parse_error(format!(
                "Unexpected token {:?}",
                next
            ))),
        }
    }

    /// Parse the expression creating an abstract syntax tree.
    fn parse(&mut self) -> Result<Expression, SyntaxError> {
        let ast = self.expression()?;
        self.assert_next(Token::End)?;
        Ok(ast)
    }
}

/// Define a lexicon to map the custom symbols from the problem to
/// the actual meaning they should have.
fn lexicon(expression: String) -> Result<Vec<Token>, SyntaxError> {
    let mut iter = expression.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut leftover: Option<char> = None;

    loop {
        let ch = match leftover {
            Some(ch) => ch,
            None => match iter.next() {
                None => break,
                Some(ch) => ch,
            },
        };
        leftover = None;
        match ch {
            ' ' => continue,
            'a' => tokens.push(Token::Plus),
            'b' => tokens.push(Token::Dash),
            'c' => tokens.push(Token::Star),
            'd' => tokens.push(Token::Slash),
            'e' => tokens.push(Token::LeftParen),
            'f' => tokens.push(Token::RightParen),
            ch if ch.is_ascii_digit() => {
                let number_stream: String = iter
                    .by_ref()
                    .take_while(|c| match c.is_ascii_digit() {
                        true => true,
                        false => {
                            leftover = Some(*c);
                            false
                        }
                    })
                    .collect();
                let number: i64 = format!("{}{}", ch, number_stream).parse().unwrap();
                tokens.push(Token::Number(number));
            }
            _ => {
                return Err(SyntaxError::new_lex_error(format!(
                    "Unrecognized character {}. Skipping it.",
                    ch
                )))
            }
        }
    }
    tokens.push(Token::End);

    Ok(tokens)
}

// Type alias some types so it's easier to write and read the code
type Output = Result<i64, Box<dyn Error>>;
type Input = String;

/// Helper function to encapsulate the logic of parsing an
/// expression, evaluating it and returning it.
fn eval_expr(expression: Input) -> Output {
    let tokens = lexicon(expression)?;
    let mut token_iter = tokens.iter().peekable();
    let mut parser = Parser::new(&mut token_iter);
    let result = parser.parse();
    match result {
        Ok(mut ast) => Ok(ast.eval()),
        Err(e) => return Err(Box::new(e)),
    }
}

fn main() {
    let testing_data = vec![
        "3a2c4",
        "32a2d2",
        "500a10b66c32",
        "3ae4c66fb32",
        "3c4d2aee2a4c41fc4f",
    ];
    let results: Vec<Output> = testing_data
        .iter()
        .map(|data| eval_expr(data.to_string()))
        .collect();

    for (input, output) in testing_data.iter().zip(results.iter()) {
        print!(
            "Input: {} => Output: {}\n",
            input.to_string(),
            output.as_ref().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test to see if basic parsing works.
    #[test]
    fn test_dummy() {
        let res = eval_expr("7".to_string());
        assert!(res.unwrap() == 7);

        let res = eval_expr("b1".to_string());
        assert!(res.unwrap() == -1);
    }

    /// Test using the given values in the problem.
    #[test]
    fn testing_data() {
        let res = eval_expr("3a2c4".to_string());
        assert!(res.unwrap() == 20);

        let res = eval_expr("32a2d2".to_string());
        assert!(res.unwrap() == 17);

        let res = eval_expr("500a10b66c32".to_string());
        assert!(res.unwrap() == 14208);

        let res = eval_expr("3ae4c66fb32".to_string());
        assert!(res.unwrap() == 235);

        let res = eval_expr("3c4d2aee2a4c41fc4f".to_string());
        assert!(res.unwrap() == 990);
    }
}
