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

/*───────────────────────────────────────────────────────────────────────────│─╗
│ ⠉⠕⠕⠇ Forth ⠉⠕⠕⠇                                                          ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

fn main() {
  if let Some(input) = Opt::from_args().input {
    println!("Input! {}", input.display());
  } else {
    println!("⠉⠕⠕⠇ Forth ⠉⠕⠕⠇");

    let mut interpreter = Interpreter::new();

    loop {
      print!("{}\n> ", interpreter.contents());

      let mut i = String::new();
      stdout().flush().expect("Coult not flush stdout.");
      stdin()
        .read_line(&mut i)
        .expect("Could not read from stdin.");

      interpreter.parse(i);
      match interpreter.exec() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
      };
    }
  }
}
