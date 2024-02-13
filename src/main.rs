mod arguments;
use arguments::*;

use clap::Parser;
use std::env;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let mut args = Arguments::default();
  
  if argv.len() > 1 {
    args = Arguments::parse();
    Arguments::run(args.debug);
  }
  
  else {
    println!("Running without args");
    Arguments::run(false);
  }
}
