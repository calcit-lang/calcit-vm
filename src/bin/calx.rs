use std::collections::hash_map::HashMap;
use std::fs;
use std::time::Instant;

use cirru_parser::{parse, Cirru};
use clap::{App, Arg};

use calx_vm::{parse_function, Calx, CalxFunc, CalxImportsDict, CalxVM};

fn main() -> Result<(), String> {
  let matches = App::new("Calx VM")
    .version("0.1.0")
    .author("Jon Chen <jiyinyiyong@gmail.com>")
    .about("A toy VM")
    .arg(
      Arg::new("SHOW_CODE")
        .short('S')
        .long("show-code")
        .value_name("show-code")
        .about("Sets a custom config file")
        .takes_value(false),
    )
    .arg(
      Arg::new("SOURCE")
        .about("A *.cirru file for loading code")
        .required(true)
        .index(1),
    )
    .get_matches();

  let source = matches.value_of("SOURCE").unwrap();
  let show_code = matches.is_present("SHOW_CODE");

  let contents = fs::read_to_string(source).expect("Cirru file for instructions");
  let code = parse(&contents).expect("Some Cirru content");

  if let Cirru::List(xs) = code {
    let mut fns: Vec<CalxFunc> = vec![];
    for x in xs {
      if let Cirru::List(ys) = x {
        let f = parse_function(&ys)?;
        fns.push(f);
      } else {
        panic!("TODO");
      }
    }

    let mut imports: CalxImportsDict = HashMap::new();
    imports.insert(String::from("log2"), (log2_calx_value, 2));

    let mut vm = CalxVM::new(fns, vec![], imports);
    if show_code {
      for func in vm.funcs.to_owned() {
        println!("loaded fn: {}", func);
      }
    }
    let now = Instant::now();

    match vm.run() {
      Ok(()) => {
        let elapsed = now.elapsed();

        println!("Took {:.3?}: {:?}", elapsed, vm.stack);
        Ok(())
      }
      Err(e) => {
        println!("VM state: {:?}", vm.stack);
        println!("{}", e);
        Err(String::from("Failed to run"))
      }
    }
  } else {
    Err(String::from("TODO not cirru code"))
  }
}

fn log2_calx_value(xs: Vec<Calx>) -> Result<Calx, String> {
  println!("log: {:?}", xs);
  Ok(Calx::Nil)
}
