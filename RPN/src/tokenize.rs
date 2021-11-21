use crate::token::Token;


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut temp = String::new();
    let mut decimal = false;

    let add_numb_token = |tkns:  &mut Vec<Token>, num: &str| -> _ {
        let t = Token::Numb(num.parse::<f64>().unwrap());
        tkns.push(t);
    };
    
    for c in input.chars() {      
        match c {
            '0' |'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => temp.push(c),
            '.' => {
                    if decimal {
                        panic!("{}", "Number contains more than 1 decimal.")
                    }
                    decimal = true;
                    temp.push(c);
                },
            _ => {
                if temp.len() > 0 {
                    add_numb_token(&mut tokens, &temp);
                    temp = String::new();
                    decimal = false;
                }
               
                match c {
                    '+' => tokens.push(Token::Add),
                    '-' => tokens.push(Token::Subtract),
                    '*'|'x'|'X' => tokens.push(Token::Multiply),
                    '/' => tokens.push(Token::Divide),
                    '^' => tokens.push(Token::Exponent),
                    '(' => tokens.push(Token::LParen),
                    ')' => tokens.push(Token::RParen),
                    ' ' => {}
                    _ => {
                        let msg = format!("Invalid char {}", c);
                        panic!("{}",msg)
                    }
                }                
            }
        }
    }

    if temp.len() > 0 {
        add_numb_token(&mut tokens, &temp);   
    }
    tokens    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_tokenizes_simple_equations() {
      let example = "3 + 5";
      let expected = vec![Token::Numb(3.0), Token::Add, Token::Numb(5.0)];
      assert_eq!(tokenize(example), expected);
    }

   #[test]
    fn it_tokenizes_complex_equations() {
        let example = "3 + 5*(7/5)^6";
        let expected = vec![
            Token::Numb(3.0), 
            Token::Add, Token::Numb(5.0),
            Token::Multiply,
            Token::LParen,
            Token::Numb(7.0),
            Token::Divide,
            Token::Numb(5.0),
            Token::RParen,
            Token::Exponent,
            Token::Numb(6.0)
            ];
        assert_eq!(tokenize(example), expected);
      }
    
    #[test]
    #[should_panic]
    fn it_panics_for_invalid_chars() {
        let example = "3&4*4";
        let _ = tokenize(example);

    }

    #[test]
    #[should_panic]
    fn it_panics_for_invalid_decimal_number() {
        let example = "3.5.4*4";
        let _ = tokenize(example);

    }
 
   #[test]
    fn it_tokenizes_large_numbers() {
        let example = "3345 * 12.34";
        let expected = vec![
            Token::Numb(3345.0),
            Token::Multiply,
            Token::Numb(12.34)
        ];
        assert_eq!(tokenize(example),expected);
    }
}
