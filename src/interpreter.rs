use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Interpreter                                                              ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Debug, Default)]
pub struct Interpreter {
  stack: Stack,
  tokens: Vec<String>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn parse(&mut self, input: String) {
    self.tokens = input.split_whitespace().map(|s| s.to_string()).collect();
  }

  pub fn exec(&mut self) -> Result<(), Error> {
    for token in &self.tokens {
      let is_num = |s: String| -> bool {
        for c in s.chars() {
          if !c.is_numeric() {
            return false;
          }
        }
        true
      };

      if is_num(token.clone()) {
        self.stack.push(token.parse::<i64>().unwrap());
        continue;
      }

      let mut op = Op::new(&mut self.stack);

      match token.as_str() {
        "+" => op.add()?,
        "-" => op.sub()?,
        "*" => op.mul()?,
        "=" => op.eq()?,
        ">" => op.gt()?,
        "<" => op.lt()?,
        "swap" => op.swap()?,
        "drop" => op.drop()?,
        "dup" => op.dup()?,
        "over" => op.over()?,
        "rot" => op.rot()?,
        "." => {
          let val = match self.stack.pop() {
            Ok(v) => v,
            Err(_) => Err(Error::StackUnderflow)?,
          };
          println!("{} ok", val);
        }
        "emit" => {
          let val = match self.stack.pop() {
            Ok(v) => v,
            Err(_) => Err(Error::StackUnderflow)?,
          };

          let ch = match u32::try_from(val) {
            Ok(v) => char::from_u32(v).unwrap(),
            Err(_) => '',
          };

          println!("{} ok", ch);
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
}
