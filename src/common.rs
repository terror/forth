// dependencies
pub(crate) use snafu::Snafu;
pub(crate) use structopt::StructOpt;

// std
pub(crate) use std::{char, collections::HashMap, convert::TryFrom, io::stdin, path::PathBuf, u32};

// modules
pub(crate) use crate::{
  error::Error,
  interpreter::Interpreter,
  op::{BinaryOperation, Op, TernaryOperation, UnaryOperation},
  stack::Stack,
  utils::Utils,
};
