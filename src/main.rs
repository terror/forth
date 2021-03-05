use crate::common::*;

mod common;
mod stack;

#[derive(StructOpt, Debug)]
#[structopt(name = "forth")]
struct Opt {
  /// Input file
  #[structopt(short, long, parse(from_os_str))]
  input: Option<PathBuf>,
}

fn main() {
  let opt = Opt::from_args();
  if let Some(input) = opt.input {
    println!("Input! {}", input.display());
  } else {
    println!("Start REPL");
  }
}
