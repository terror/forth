use crate::common::*;

/*───────────────────────────────────────────────────────────────────────────│─╗
│ Operation Types                                                          ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

pub enum UnaryOperation {
  Drop,
  Dup,
  Emit,
  Dot,
  Cr,
}
pub enum BinaryOperation {
  Add,
  Sub,
  Mul,
  Eq,
  Gt,
  Lt,
  Swap,
  Over,
}
pub enum TernaryOperation {
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

  pub fn unary(&mut self, op: UnaryOperation) -> Result<(), Error> {
    let first = self.state.pop()?;

    match op {
      UnaryOperation::Cr => {
        self.state.push(first);
        print!("\n");
      },
      UnaryOperation::Dup => {
        self.state.push(first);
        self.state.push(first)
      },
      UnaryOperation::Dot => println!("{} ok", first),
      UnaryOperation::Emit => println!("{} ok", match u32::try_from(first) {
        Ok(v) => char::from_u32(v).unwrap(),
        Err(_) => '',
      }),
      UnaryOperation::Drop => {},
    }

    Ok(())
  }

  pub fn binary(&mut self, op: BinaryOperation) -> Result<(), Error> {
    let first = self.state.pop()?;
    let second = self.state.pop()?;

    match op {
      BinaryOperation::Add => self.state.push(first + second),
      BinaryOperation::Sub => self.state.push(first - second),
      BinaryOperation::Mul => self.state.push(first * second),
      BinaryOperation::Eq => self.state.push(match first == second {
        true => -1,
        false => 0,
      }),
      BinaryOperation::Gt => self.state.push(match first < second {
        true => -1,
        false => 0,
      }),
      BinaryOperation::Lt => self.state.push(match first > second {
        true => -1,
        false => 0,
      }),
      BinaryOperation::Swap => {
        self.state.push(first);
        self.state.push(second);
      },
      BinaryOperation::Over => {
        self.state.push(second);
        self.state.push(first);
        self.state.push(second);
      },
    }

    Ok(())
  }

  pub fn ternary(&mut self, op: TernaryOperation) -> Result<(), Error> {
    let first = self.state.pop()?;
    let second = self.state.pop()?;
    let third = self.state.pop()?;

    match op {
      TernaryOperation::Rot => {
        self.state.push(second);
        self.state.push(first);
        self.state.push(third);
      },
    }

    Ok(())
  }
}
