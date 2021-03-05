#[derive(Default)]
pub struct Stack {
  pub v: Vec<i64>,
}

impl Stack {
  pub fn new() -> Self {
    Stack::default()
  }

  pub fn push(&mut self, element: i64) {
    self.v.push(element);
  }

  pub fn pop(&mut self) -> i64 {
    if let Some(v) = self.v.pop() {
      v
    } else {
      -1
    }
  }

  pub fn peek(&mut self) -> i64 {
    if self.is_empty() {
      return -1;
    }
    self.v[self.v.len() - 1]
  }

  pub fn is_empty(&mut self) -> bool {
    self.size() == 0
  }

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
    let val = s.pop();
    assert_eq!(2, val);
    assert_eq!(1, s.size());
  }

  #[test]
  fn test_pop_empty() {
    let mut s: Stack = Stack::new();
    s.push(1);
    s.push(2);

    let val = s.pop();
    assert_eq!(2, val);
    assert_eq!(1, s.size());

    let val = s.pop();
    assert_eq!(1, val);
    assert!(s.is_empty());

    let val = s.pop();
    assert_eq!(-1, val);
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
    s.pop();
    assert!(s.is_empty());

    let val = s.peek();
    assert_eq!(-1, val);
  }
}
