
mod evaluate;
mod postfix;
mod token;
mod tokenize;


pub fn solve(expression: &str) -> f64 {
    let tokens = tokenize::tokenize(expression);
    let postfix = postfix::convert_to_postfix(tokens);
    let ans = evaluate::evaluate_postfix(postfix);
    
    ans
}



