use Calculator;

#[test]
fn solve_simple_equation() {
  let example = "3* 5";
  assert_eq!(Calculator::solve(example), 15.0);
}

#[test]
fn solve_complex_equation() {
    let example = " 5 + 2 / (3- 8) ^ 5 ^ 2";
    assert_eq!(Calculator::solve(example).round(), 5.0);
  }

  #[test]
fn solve_large_numbers_equation() {
    let example = " 500+300*850";
    assert_eq!(Calculator::solve(example), 255500.0);
}