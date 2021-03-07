use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Stack                                                                    ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Default, Debug)]
pub struct Stack {
  v: Vec<i64>,
}

impl Stack {
  #[cfg(test)]
  pub fn new() -> Self {
    Self::default()
  }

  pub fn push(&mut self, element: i64) {
    self.v.push(element);
  }

  pub fn pop(&mut self) -> Result<i64, Error> {
    if let Some(v) = self.v.pop() {
      Ok(v)
    } else {
      Err(Error::StackUnderflow)
    }
  }

  pub fn contents(&mut self) -> &Vec<i64> {
    &self.v
  }

  #[cfg(test)]
  pub fn peek(&mut self) -> i64 {
    if self.is_empty() {
      return -1;
    }
    self.v[self.v.len() - 1]
  }

  #[cfg(test)]
  pub fn is_empty(&mut self) -> bool {
    self.size() == 0
  }

  #[cfg(test)]
  pub fn size(&mut self) -> i64 {
    self.v.len() as i64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_push() {
    let mut s: Stack = Stack::new();
    s.push(1);
    assert_eq!(1, s.size());
  }

  #[test]
  fn test_pop() {
    let mut s: Stack = Stack::new();
    s.push(1);
    s.push(2);
    let val = s.pop().unwrap();
    assert_eq!(2, val);
    assert_eq!(1, s.size());
  }

  #[test]
  fn test_pop_empty() {
    let mut s: Stack = Stack::new();
    s.push(1);
    s.push(2);

    let val = s.pop().unwrap();
    assert_eq!(2, val);
    assert_eq!(1, s.size());

    let val = s.pop().unwrap();
    assert_eq!(1, val);
    assert!(s.is_empty());

    let result = s.pop();
    assert!(result.is_err());
  }

  #[test]
  fn test_peek() {
    let mut s: Stack = Stack::new();
    s.push(1);
    s.push(2);

    let val = s.peek();
    assert_eq!(2, val);
  }

  #[test]
  fn test_peek_empty() {
    let mut s: Stack = Stack::new();
    s.push(1);
    s.pop().unwrap();
    assert!(s.is_empty());

    let val = s.peek();
    assert_eq!(-1, val);
  }
}
