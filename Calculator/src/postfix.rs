use crate::token::Token;

use std::collections::VecDeque;
/*
    https://brilliant.org/wiki/shunting-yard-algorithm/
    https://www.geeksforgeeks.org/java-program-to-implement-shunting-yard-algorithm/

    1 stack for operations
    1 queue of the output
    1 array (or other list) of tokens.

    1.  While there are tokens to be read:
    2.        Read a token
    3.        If it's a number add it to queue
    4.        If it's an operator
    5.               While there's an operator on the top of the stack with equal or greater  precedence:
    6.                       Pop operators from the stack onto the output queue
    7.               Push the current operator onto the stack
    8.        If it's a left bracket push it onto the stack
    9.        If it's a right bracket 
    10.            While there's not a left bracket at the top of the stack:
    11.                     Pop operators from the stack onto the output queue.
    12.             Pop the left bracket from the stack and discard it
    13. While there are operators on the stack, pop them to the queue
*/

pub fn convert_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators: VecDeque<Token> = VecDeque::new();

    for token in tokens {
        match token {
            Token::Numb(_) => output.push(token),
            Token::Add|
            Token::Subtract|
            Token::Multiply|
            Token::Divide|
            Token::Exponent => {
                let tok_pres = get_precedence(&token);
                let mut optional = operators.pop_front();
                while let Some(t) = optional {
                    let pres = get_precedence(&t);
                    if pres >= tok_pres {                      
                        output.push(t);
                        optional = operators.pop_front();
                    } else {
                        operators.push_front(t);
                        optional = None;
                    }
                }
                operators.push_front(token);
            },
            Token::LParen => operators.push_front(token),
            Token::RParen => {
                let mut optional = operators.pop_front();

                while let Some(t) = optional {
                    if t == Token::LParen {
                        optional = None;
                    } else {
                        output.push(t);
                        optional = operators.pop_front();
                    }
                }
            }
        }
    }

    let mut optional = operators.pop_front();
    while let Some(t) = optional {
        output.push(t);
        optional = operators.pop_front();
    }

    output
}

fn get_precedence(operator: &Token) -> u32 {
    match operator {
        Token::Add | Token::Subtract => return 1,
        Token::Multiply | Token::Divide => return 2,
        Token::Exponent => return 3,
        _ => return 0
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_postfix_single_operator() {
        let example = vec![
            Token::Numb(4.0),
            Token::Add,
            Token::Numb(8.0)
        ];

        let expected = vec![
            Token::Numb(4.0),
            Token::Numb(8.0),
            Token::Add
        ];

        assert_eq!(convert_to_postfix(example), expected);
    }

    #[test]
    fn it_converts_to_postfix_2_operators() {       
        let example = vec![
            Token::Numb(2.0),
            Token::Add,
            Token::Numb(3.0),
            Token::Multiply,
            Token::Numb(4.0)
        ];

        let expected = vec![
            Token::Numb(2.0),
            Token::Numb(3.0),
            Token::Numb(4.0),
            Token::Multiply,
            Token::Add
        ];

        assert_eq!(convert_to_postfix(example), expected);
    }

    #[test]
    fn it_converts_to_postfix_complex_equation() {
        // 5 + 2 / (3- 8) ^ 5 ^ 2 => 5238-5^2^/+
        let example = vec![
            Token::Numb(5.0),
            Token::Add,
            Token::Numb(2.0),
            Token::Divide,
            Token::LParen,
            Token::Numb(3.0),
            Token::Subtract,
            Token::Numb(8.0),
            Token::RParen,
            Token::Exponent,
            Token::Numb(5.0),
            Token::Exponent,
            Token::Numb(2.0)
        ];

        let expected = vec![
            Token::Numb(5.0),
            Token::Numb(2.0),
            Token::Numb(3.0),
            Token::Numb(8.0),
            Token::Subtract,
            Token::Numb(5.0),
            Token::Exponent,
            Token::Numb(2.0),
            Token::Exponent,
            Token::Divide,
            Token::Add
        ];

        assert_eq!(convert_to_postfix(example), expected);
    }

}