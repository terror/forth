use crate::common::*;

mod common;
mod error;
mod interpreter;
mod op;
mod stack;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "forth")]
struct Opt {
  /// Input file
  #[structopt(short, long, parse(from_os_str))]
  input: Option<PathBuf>,
}

fn clear() {
  // clear terminal screen
  print!("{esc}c", esc = 27 as char);
}

fn main() {
  if let Some(input) = Opt::from_args().input {
    println!("Input! {}", input.display());
  } else {
    let mut interpreter = Interpreter::new();

    clear();

    loop {
      println!("⠉⠕⠕⠇ Forth ⠉⠕⠕⠇");

      println!("{}", interpreter.contents());

      let mut i = String::new();

      stdin().read_line(&mut i).unwrap();
      clear();

      match interpreter.parse(i).exec() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
      };
    }
  }
}
