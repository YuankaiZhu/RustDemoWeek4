use std::str::FromStr;

fn main() {
    let mut input = String::new();
    println!("Enter an expression:");
    std::io::stdin().read_line(&mut input).unwrap();

    let result = eval(&input).unwrap();
    println!("Result: {}", result);
}

fn eval(input: &str) -> Result<i32, &'static str> {
    let mut tokens = tokenize(input);
    let mut result = parse_expression(&mut tokens)?;
    if !tokens.is_empty() {
        return Err("Invalid expression");
    }
    Ok(result)
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();
    for ch in input.trim().chars() {
        match ch {
            '0'..='9' => number.push(ch),
            '+' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(i32::from_str(&number).unwrap()));
                    number.clear();
                }
                tokens.push(Token::Plus);
            }
            '-' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(i32::from_str(&number).unwrap()));
                    number.clear();
                }
                tokens.push(Token::Minus);
            }
            '*' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(i32::from_str(&number).unwrap()));
                    number.clear();
                }
                tokens.push(Token::Multiply);
            }
            '/' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(i32::from_str(&number).unwrap()));
                    number.clear();
                }
                tokens.push(Token::Divide);
            }
            '(' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(i32::from_str(&number).unwrap()));
                    number.clear();
                }
                tokens.push(Token::OpenParenthesis);
            }
            ')' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(i32::from_str(&number).unwrap()));
                    number.clear();
                }
                tokens.push(Token::CloseParenthesis);
            }
            _ => return vec![],
        }
    }
    if !number.is_empty() {
        tokens.push(Token::Number(i32::from_str(&number).unwrap()));
    }
    tokens
}

fn parse_expression(tokens: &mut Vec<Token>) -> Result<i32, &'static str> {
    let mut result = parse_term(tokens)?;
    while tokens.len() > 0 {
        match tokens[0] {
            Token::Plus => {
                tokens.remove(0);
                result += parse_term(tokens)?;
            }
            Token::Minus => {
                tokens.remove(0);
                result -= parse_term(tokens)?;
            }
            _ => break,
        }
    }
    Ok(result)
}

fn parse_term(tokens: &mut Vec<Token>) -> Result<i32, &'static str> {
    let mut result = parse_factor(tokens)?;
    while tokens.len() > 0 {
        match tokens[0] {
            Token::Multiply => {
                tokens.remove(0);
                result *= parse_factor(tokens)?;
            }
            Token::Divide => {
                tokens.remove(0);
                result /= parse_factor(tokens)?;
            }
            _ => break,
        }
    }
    Ok(result)
}

fn parse_factor(tokens: &mut Vec<Token>) -> Result<i32, &'static str> {
    match tokens[0] {
        Token::Number(number) => {
            tokens.remove(0);
            Ok(number)
        }
        Token::OpenParenthesis => {
            tokens.remove(0);
            let result = parse_expression(tokens)?;
            if tokens.len() == 0 || tokens[0] != Token::CloseParenthesis {
                return Err("Mismatched parentheses");
            }
            tokens.remove(0);
            Ok(result)
        }
        _ => Err("Invalid expression"),
    }
}
