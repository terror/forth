use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Op                                                                        ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

#[derive(Debug)]
pub struct Op<'a> {
  state: &'a mut Stack,
}

impl<'a> Op<'a> {
  pub fn new(state: &'a mut Stack) -> Self {
    Self { state }
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
      },
      BinaryOperation::Lt => self.state.push(match first > second {
        true => -1,
        false => 0,
      }),
    }

    Ok(())
  }

  pub fn add(&mut self) -> Result<(), Error> {
    self.binary(BinaryOperation::Add)?;
    Ok(())
  }

  pub fn sub(&mut self) -> Result<(), Error> {
    self.binary(BinaryOperation::Sub)?;
    Ok(())
  }

  pub fn mul(&mut self) -> Result<(), Error> {
    self.binary(BinaryOperation::Mul)?;
    Ok(())
  }

  pub fn eq(&mut self) -> Result<(), Error> {
    self.binary(BinaryOperation::Eq)?;
    Ok(())
  }

  pub fn gt(&mut self) -> Result<(), Error> {
    self.binary(BinaryOperation::Gt)?;
    Ok(())
  }

  pub fn lt(&mut self) -> Result<(), Error> {
    self.binary(BinaryOperation::Lt)?;
    Ok(())
  }
}
