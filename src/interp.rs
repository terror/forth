use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Interpreter                                                              ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Default, Debug)]
pub struct Interp {
  pub stack: Stack,
  pub tokens: Vec<String>,
}

impl Interp {
  pub fn new() -> Self {
    Interp::default()
  }

  pub fn parse(&mut self, input: String) {
    self.tokens = input.split_whitespace().map(|s| s.to_string()).collect();
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

      match token.as_str() {
        "+" => self.stack.add()?,
        "-" => self.stack.sub()?,
        "*" => self.stack.mul()?,
        "." => {
          let val = match self.stack.pop() {
            Ok(v) => v,
            Err(_) => panic!("err"),
          };
          println!("{} ok", val);
        }
        "=" => self.stack.eq()?,
        ">" => self.stack.gt()?,
        "<" => self.stack.lt()?,
        "emit" => {
          let val = match self.stack.emit() {
            Ok(v) => v,
            Err(_) => panic!("err"),
          };
          println!("{} ok", val);
        }
        _ => panic!("Invalid operation!"),
      }
    }
    Ok(())
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
