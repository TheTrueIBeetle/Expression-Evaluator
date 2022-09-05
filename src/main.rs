/*
 * Project Name: expression_evaluator
 * File Name: main.rs
 * Author: Luke Bas
 * Date Created: 2022-08-27
*/

use std::io;

use expression_evaluator::evaluator::*;

fn main() {

    loop {
        let mut v: Vec<Token> = Vec::new();
        println!("Input an equation or 'exit' to exit: ");
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line..");
        if s == String::from("exit") {
            break;
        }

        let len = s.len();
        s.truncate(len - 2);

        for substring in s
            .split(' ')
            .filter(|s| s.len() > 0)  
            {
                v.push(tokenize_string(substring).unwrap());
            }

        //convert to reverse polish notation
        let rpn_vector: Vec<Token> = convert_to_rpn(v);

        //make new string from the rpn vector
        let mut new_str = String::new();
        for tok in rpn_vector {
            new_str.push_str(&stringify_token(tok).unwrap());
            new_str.push_str(&String::from(" "));
        }

        println!("\n");

        let mut tokens = lex(&new_str).unwrap();
        tokens.reverse();
        let result = evaluate(&tokens).unwrap();
        println!("Result: {:?}", result);
    }

    
}


