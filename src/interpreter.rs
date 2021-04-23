use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Interpreter                                                              ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Debug, Default)]
pub struct Interpreter {
  stack: Stack,
  tokens: Vec<String>,
  variables: HashMap<String, String>,
  ptr: i64,
}

impl Interpreter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn parse(&mut self, input: String) {
    self.tokens = Utils::split(input);
    self.reset();
  }

  pub fn get_next(&mut self) -> Result<String, Error> {
    if self.has_next() {
      let ret = self.tokens[self.ptr as usize].clone();
      self.ptr += 1;
      Ok(ret)
    } else {
      Err(Error::TokenOutOfBounds)
    }
  }

  pub fn peek_next(&mut self) -> Result<String, Error> {
    if self.has_next() {
      let ret = self.tokens[self.ptr as usize].clone();
      Ok(ret)
    } else {
      Err(Error::TokenOutOfBounds)
    }
  }

  pub fn has_next(&mut self) -> bool {
    self.ptr < (self.tokens.len() as i64)
  }

  pub fn reset(&mut self) {
    self.ptr = 0;
  }

  pub fn exec(&mut self) -> Result<(), Error> {
    while self.has_next() {
      let token = self.get_next()?;

      if Utils::is_num(&token) {
        self.stack.push(token.parse::<i64>().unwrap());
        continue;
      }

      if let Some(val) = self.variables.get(&token) {
        self.tokens.append(&mut Utils::split(val.to_string()));
        continue;
      }

      let mut op = Op::new(&mut self.stack);

      match token.as_str() {
        "dup" => op.unary(UnaryOperation::Dup)?,
        "over" => op.binary(BinaryOperation::Over)?,
        "emit" => op.unary(UnaryOperation::Emit)?,
        "cr" => op.unary(UnaryOperation::Cr)?,
        "drop" => op.unary(UnaryOperation::Drop)?,
        "." => op.unary(UnaryOperation::Dot)?,
        "+" => op.binary(BinaryOperation::Add)?,
        "-" => op.binary(BinaryOperation::Sub)?,
        "*" => op.binary(BinaryOperation::Mul)?,
        "=" => op.binary(BinaryOperation::Eq)?,
        ">" => op.binary(BinaryOperation::Gt)?,
        "<" => op.binary(BinaryOperation::Lt)?,
        "swap" => op.binary(BinaryOperation::Swap)?,
        "rot" => op.ternary(TernaryOperation::Rot)?,
        ";" => {}
        ":" => {
          let name = self.get_next()?;

          let mut value = Vec::new();
          while self.peek_next()? != ";" {
            value.push(self.get_next()?);
          }

          self.variables.insert(name, value.join(" "));
        }
        _ => Err(Error::NotFound)?,
      }
    }

    Ok(())
  }

  pub fn contents(&mut self) -> String {
    self
      .stack
      .contents()
      .into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .join(" ")
      + " <- Top"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_push() {
    let mut interpreter = Interpreter::new();
    let input = String::from("1");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_dot() {
    let mut interpreter = Interpreter::new();
    let input = String::from("1 1 .");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_add() {
    let mut interpreter = Interpreter::new();
    let input = String::from("1 1 1");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 3);

    let input = String::from("+");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 2);
    assert_eq!(interpreter.stack.pop().unwrap(), 2);
  }

  #[test]
  fn test_sub() {
    let mut interpreter = Interpreter::new();
    let input = String::from("1 1 1");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 3);

    let input = String::from("-");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 2);
    assert_eq!(interpreter.stack.pop().unwrap(), 0);
  }

  #[test]
  fn test_mul() {
    let mut interpreter = Interpreter::new();
    let input = String::from("1 1 1");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 3);

    let input = String::from("*");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 2);
    assert_eq!(interpreter.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_eq() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 1 =");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), -1);

    let input = String::from("1 2 =");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 0);
  }

  #[test]
  fn test_gt() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 2 >");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 0);

    let input = String::from("2 1 >");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), -1);
  }

  #[test]
  fn test_lt() {
    let mut interpreter = Interpreter::new();

    let input = String::from("2 1 <");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 0);

    let input = String::from("1 2 <");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), -1);
  }

  #[test]
  fn test_swap() {
    let mut interpreter = Interpreter::new();

    let input = String::from("2 1 swap");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 2);
    assert_eq!(interpreter.stack.pop().unwrap(), 2);
  }

  #[test]
  fn test_rot() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 2 3 rot");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 3);
    assert_eq!(interpreter.stack.pop().unwrap(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 3);
    assert_eq!(interpreter.stack.pop().unwrap(), 2);
  }

  #[test]
  fn test_dup() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 2 3 dup");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 4);
    assert_eq!(interpreter.stack.pop().unwrap(), 3);
  }

  #[test]
  fn test_over() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 2 3 over");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 4);
    assert_eq!(interpreter.stack.pop().unwrap(), 2);
  }

  #[test]
  fn test_drop() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 2 3 drop");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 2);
    assert_eq!(interpreter.stack.pop().unwrap(), 2);
  }

  #[test]
  fn test_push_after_underflow() {
    let mut interpreter = Interpreter::new();

    let input = String::from("1 . .");
    interpreter.parse(input);
    let result = interpreter.exec();
    assert!(result.is_err());

    let input = String::from("1");
    interpreter.parse(input);
    interpreter.exec().unwrap();
    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_variable() {
    let mut interpreter = Interpreter::new();

    let input = String::from(": foo 100 ;");
    interpreter.parse(input);
    let result = interpreter.exec();
    assert!(result.is_ok(), result.err().unwrap().to_string());

    let input = String::from("foo");
    interpreter.parse(input);
    interpreter.exec().unwrap();

    assert_eq!(interpreter.stack.size(), 1);
    assert_eq!(interpreter.stack.pop().unwrap(), 100);
  }

  #[test]
  fn test_multiple_variables() {
    let mut interpreter = Interpreter::new();

    let input = String::from(": foo 100 ; : bar 100 ;");
    interpreter.parse(input);
    let result = interpreter.exec();
    assert!(result.is_ok(), result.err().unwrap().to_string());

    let input = String::from("foo bar");
    interpreter.parse(input);
    interpreter.exec().unwrap();

    assert_eq!(interpreter.stack.size(), 2);
    assert_eq!(interpreter.stack.pop().unwrap(), 100);
    assert_eq!(interpreter.stack.pop().unwrap(), 100);
  }
}
