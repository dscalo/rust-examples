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
    5.               While there's an operator on the top of the stack with greater precedence:
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
    let mut postfix = Vec::new();

    let mut output = Vec::new();
    let mut operators: VecDeque<Token> = VecDeque::new();

    for token in tokens {
        match token {
            Token::Numb(_) => output.push(token),
            Token::Add|Token::Subtract => {
                while operators.front()
            }
            _ => {}
        }
    }

    
    postfix
}

fn get_precedence(operator: Token) -> u32 {
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
    fn it_converts_tokens_to_postfix_simple() {
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

}