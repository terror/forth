use crate::common::*;

mod common;
mod error;
mod interp;
mod op;
mod stack;
mod types;

#[derive(StructOpt, Debug)]
#[structopt(name = "forth")]
struct Opt {
  /// Input file
  #[structopt(short, long, parse(from_os_str))]
  input: Option<PathBuf>,
}

/*───────────────────────────────────────────────────────────────────────────│─╗
│ ⠉⠕⠕⠇ 4 ⠉⠕⠕⠇                                                               ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

fn main() {
  if let Some(input) = Opt::from_args().input {
    println!("Input! {}", input.display());
  } else {
    println!("⠉⠕⠕⠇ 4 ⠉⠕⠕⠇");

    let mut interp = Interp::new();

    loop {
      print!("{}\n> ", interp.contents());

      let mut i = String::new();
      stdout().flush().expect("Coult not flush stdout.");
      stdin()
        .read_line(&mut i)
        .expect("Could not read from stdin.");

      interp.parse(i);
      match interp.exec() {
        Ok(_) => {},
        Err(e) => println!("{}", e),
      };
    }
  }
}
