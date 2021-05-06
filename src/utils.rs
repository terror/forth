pub struct Utils;

impl Utils {
  pub fn split(input: String) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
  }

  pub fn is_num(s: &str) -> bool {
    for c in s.chars() {
      if !c.is_numeric() {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_num() {
    assert!(Utils::is_num("1234"));
    assert_eq!(Utils::is_num("123J4"), false);
  }

  #[test]
  fn test_split() {
    assert_eq!(
      Utils::split(String::from(": foo 1 ;")),
      vec![":", "foo", "1", ";"]
    );
  }
}
