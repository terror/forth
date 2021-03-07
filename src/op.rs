use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Operation Types                                                          ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

enum UnaryOperation {
  Drop,
  Dup,
}

enum BinaryOperation {
  Add,
  Sub,
  Mul,
  Eq,
  Gt,
  Lt,
  Swap,
  Over,
}

enum TernaryOperation {
  Rot,
}

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Op                                                                       ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Debug)]
pub struct Op<'a> {
  state: &'a mut Stack,
}

impl<'a> Op<'a> {
  pub fn new(state: &'a mut Stack) -> Self {
    Self { state }
  }

  fn unary(&mut self, t: UnaryOperation) -> Result<(), Error> {
    let first = self.state.pop()?;

    match t {
      UnaryOperation::Drop => {}
      UnaryOperation::Dup => {
        self.state.push(first);
        self.state.push(first);
      }
    }

    Ok(())
  }

  fn binary(&mut self, t: BinaryOperation) -> Result<(), Error> {
    let first = self.state.pop()?;
    let second = self.state.pop()?;

    match t {
      BinaryOperation::Add => self.state.push(first + second),
      BinaryOperation::Sub => self.state.push(first - second),
      BinaryOperation::Mul => self.state.push(first * second),
      BinaryOperation::Eq => self.state.push(match first == second {
        true => -1,
        false => 0,
      }),
      BinaryOperation::Gt => {
        self.state.push(match first < second {
          true => -1,
          false => 0,
        });
      }
      BinaryOperation::Lt => self.state.push(match first > second {
        true => -1,
        false => 0,
      }),
      BinaryOperation::Swap => {
        self.state.push(first);
        self.state.push(second);
      }
      BinaryOperation::Over => {
        self.state.push(second);
        self.state.push(first);
        self.state.push(second);
      }
    }

    Ok(())
  }

  fn ternary(&mut self, t: TernaryOperation) -> Result<(), Error> {
    let first = self.state.pop()?;
    let second = self.state.pop()?;
    let third = self.state.pop()?;

    match t {
      TernaryOperation::Rot => {
        self.state.push(second);
        self.state.push(first);
        self.state.push(third);
      }
    }

    Ok(())
  }

  pub fn drop(&mut self) -> Result<(), Error> {
    Ok(self.unary(UnaryOperation::Drop)?)
  }

  pub fn dup(&mut self) -> Result<(), Error> {
    Ok(self.unary(UnaryOperation::Dup)?)
  }

  pub fn add(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Add)?)
  }

  pub fn sub(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Sub)?)
  }

  pub fn mul(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Mul)?)
  }

  pub fn eq(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Eq)?)
  }

  pub fn gt(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Gt)?)
  }

  pub fn lt(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Lt)?)
  }

  pub fn swap(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Swap)?)
  }

  pub fn over(&mut self) -> Result<(), Error> {
    Ok(self.binary(BinaryOperation::Over)?)
  }

  pub fn rot(&mut self) -> Result<(), Error> {
    Ok(self.ternary(TernaryOperation::Rot)?)
  }
}
