/*
 * Project Name: expression_evaluator
 * File Name: lib.rs
 * Author: Luke Bas
 * Date Created: 2022-08-27
*/

pub mod evaluator {

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Operation {
        Add,
        Sub,
        Mul,
        Div,
        Number(i32),
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Token {
        Number(i32),
        Op(Operation),
        Bracket(char),
    }

    #[derive(Debug)]
    pub enum Error {
        InvalidToken(String),
        InvalidProgram,
        InvalidString,
    }

    pub fn lex(code: &str) -> Result<Vec<Operation>, Error> {
        let mut tokens = vec![];
    
        for substring in code
        .split(' ')
        .filter(|s| s.len() > 0) 
        {
            match substring {
                "+" => tokens.push(Operation::Add),
                "-" => tokens.push(Operation::Sub),
                "*" => tokens.push(Operation::Mul),
                "/" => tokens.push(Operation::Div),
                n => 
                    if let Ok(n) = n.parse::<i32>() {
                        tokens.push(Operation::Number(n));
                    }
                    else {
                        return Err(Error::InvalidToken(substring.to_owned()));
                    },
            }
        }
        Ok(tokens)
    }

    pub fn evaluate(tokens: &[Operation]) -> Result<(i32, usize), Error> {
        if let Some(first_token) = tokens.first() {
            match first_token {
                Operation::Add => {
                    let (a, a_skip) = evaluate(&tokens[1..])?;
                    let (b, b_skip) = evaluate(&tokens[1 + a_skip..])?;
                    Ok((a + b, 1 + a_skip + b_skip))
                },
                Operation::Sub => {
                    let (a, a_skip) = evaluate(&tokens[1..])?;
                    let (b, b_skip) = evaluate(&tokens[1 + a_skip..])?;
                    Ok((a - b, 1 + a_skip + b_skip))
                },
                Operation::Mul => {
                    let (a, a_skip) = evaluate(&tokens[1..])?;
                    let (b, b_skip) = evaluate(&tokens[1 + a_skip..])?;
                    Ok((a * b, 1 + a_skip + b_skip))
                },
                Operation::Div => {
                    let (a, a_skip) = evaluate(&tokens[1..])?;
                    let (b, b_skip) = evaluate(&tokens[1 + a_skip..])?;
                    Ok((a / b, 1 + a_skip + b_skip))
                },
                Operation::Number(n) => Ok((*n, 1)), //dereference this from ref to i32 to i32
            }
        }
        else {
            Err(Error::InvalidProgram)
        }
    } 

    pub fn convert_to_rpn(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => { 
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {}
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }

        return queue;

    }

    pub fn tokenize_string(input: &str) -> Result<Token, Error> {
        match input {
            "+" => return Ok(Token::Op(Operation::Add)),
            "-" => return Ok(Token::Op(Operation::Sub)),
            "*" => return Ok(Token::Op(Operation::Mul)),
            "/" => return Ok(Token::Op(Operation::Div)),
            ")" => return Ok(Token::Bracket(')')),
            "(" => return Ok(Token::Bracket('(')),

            n => {
                if let Ok(n) = n.parse::<i32>() {
                    return Ok(Token::Number(n));
                }
                else {
                    return Err(Error::InvalidToken(n.to_owned()));
                }
                
            }
        }
    }

    pub fn stringify_token(input: Token) -> Result<String, Error> {
        match input {
            Token::Op(Operation::Add) => return Ok(String::from("+")),
            Token::Op(Operation::Sub) => return Ok(String::from("-")),
            Token::Op(Operation::Mul) => return Ok(String::from("*")),
            Token::Op(Operation::Div) => return Ok(String::from("/")),
            Token::Number(_input) => return Ok(_input.to_string()),
            Token::Bracket('(') => return Ok(String::from("(")),
            Token::Bracket(')') => return Ok(String::from(")")),
            
            _ => {
                return Err(Error::InvalidString);
            }
        }
    }


}