use crate::token::Token;

use std::collections::VecDeque;

pub fn evaluate_postfix(tokens: Vec<Token>) -> f64 {
  let mut stack: VecDeque<f64> = VecDeque::new();

  for token in tokens {
    match token {
      Token::Numb(n) => stack.push_front(n),
      Token::Add | Token::Subtract | Token::Multiply | Token::Divide | Token::Exponent => {
        let r_val = stack.pop_front();
        let l_val = stack.pop_front();

        match (r_val, l_val) {
          (Some(r), Some(l)) => {
            let val = calculate(r, l, token);
            stack.push_front(val);
          }
          _ => panic!("{}", "Invalid stack"),
        }
      }
      _ => panic!("{}", "invalid token"),
    }
  }

  0.0
}

fn calculate(r: f64, l: f64, operator: Token) -> f64 {
  match (operator) {
    Token::Add => return r + l,
    Token::Subtract => return r - l,
    Token::Multiply => return r * l,
    Token::Divide => return r / l,
    Token::Exponent => return f64::powf(r, l),
    _ => return 0.0,
  }
}
