use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Interpreter                                                               ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Debug, Default)]
pub struct Interp {
  stack:  Stack,
  tokens: Vec<String>,
}

impl Interp {
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
        "." => {
          let val = match self.stack.pop() {
            Ok(v) => v,
            Err(_) => Err(Error::StackUnderflow)?,
          };
          println!("{} ok", val);
        },
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
        },
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
    let mut interp = Interp::new();
    let input = String::from("1");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_dot() {
    let mut interp = Interp::new();
    let input = String::from("1 1 .");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_add() {
    let mut interp = Interp::new();
    let input = String::from("1 1 1");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 3);

    let input = String::from("+");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 2);
    assert_eq!(interp.stack.pop().unwrap(), 2);
  }

  #[test]
  fn test_sub() {
    let mut interp = Interp::new();
    let input = String::from("1 1 1");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 3);

    let input = String::from("-");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 2);
    assert_eq!(interp.stack.pop().unwrap(), 0);
  }

  #[test]
  fn test_mul() {
    let mut interp = Interp::new();
    let input = String::from("1 1 1");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 3);

    let input = String::from("*");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 2);
    assert_eq!(interp.stack.pop().unwrap(), 1);
  }

  #[test]
  fn test_eq() {
    let mut interp = Interp::new();

    let input = String::from("1 1 =");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), -1);

    let input = String::from("1 2 =");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), 0);
  }

  #[test]
  fn test_gt() {
    let mut interp = Interp::new();

    let input = String::from("1 2 >");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), 0);

    let input = String::from("2 1 >");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), -1);
  }

  #[test]
  fn test_lt() {
    let mut interp = Interp::new();

    let input = String::from("2 1 <");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), 0);

    let input = String::from("1 2 <");
    interp.parse(input);
    interp.exec().unwrap();
    assert_eq!(interp.stack.size(), 1);
    assert_eq!(interp.stack.pop().unwrap(), -1);
  }
}
