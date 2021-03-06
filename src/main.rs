use crate::common::*;

mod common;
mod error;
mod interp;
mod stack;

#[derive(StructOpt, Debug)]
#[structopt(name = "forth")]
struct Opt {
  /// Input file
  #[structopt(short, long, parse(from_os_str))]
  input: Option<PathBuf>,
}

/*───────────────────────────────────────────────────────────────────────────│─╗
│ ⠉⠕⠕⠇ 4 ⠉⠕⠕⠇                                                              ─╬─│┼
╚────────────────────────────────────────────────────────────────────────────│*/

fn main() {
  if let Some(input) = Opt::from_args().input {
    println!("Input! {}", input.display());
  } else {
    println!("⠉⠕⠕⠇ 4 ⠉⠕⠕⠇");
    let mut interp = Interp::new();
    loop {
      println!("{}", interp.contents());
      print!("> ");

      let mut i = String::new();
      stdout().flush().unwrap();
      stdin().read_line(&mut i).unwrap();

      interp.parse(i);
      match interp.exec() {
        Ok(_) => {},
        Err(e) => println!("{}", e),
      };
    }
  }
}
