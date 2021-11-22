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

        match (l_val, r_val) {
          (Some(l), Some(r)) => {
            let val = calculate(l, r, token);
            stack.push_front(val);
          }
          _ => panic!("{}", "Invalid stack"),
        }
      }
      _ => panic!("{}", "invalid token"),
    }
  }

  let res = stack.pop_front();
  if stack.len() > 0 {
    panic!("{}", "Stack contains additional values");
  }
  if let Some(val) = res {
    return val;
  } else {
    panic!("{}", "Invalid stack");
  }
}

fn calculate(l: f64, r: f64, operator: Token) -> f64 {
  match operator {
    Token::Add => return l + r,
    Token::Subtract => return l - r,
    Token::Multiply => return l * r,
    Token::Divide => return l / r,
    Token::Exponent => return f64::powf(l, r),
    _ => return 0.0,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_evaluates_adding_2_numbers() {
    let example = vec![Token::Numb(2.0), Token::Numb(4.0), Token::Add];

    assert_eq!(evaluate_postfix(example), 6.0);
  }

  #[test]
  fn it_evaluates_complex_equation() {
    let example = vec![
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
      Token::Add,
    ];

    assert_eq!(evaluate_postfix(example).round(), 5.0);
  }
}
