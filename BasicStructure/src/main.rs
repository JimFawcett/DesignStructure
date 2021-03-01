// basic_structure::main.rs

use std::fs::*;
use std::io::{Read, Error, ErrorKind};

fn open_file_for_read(file_name:&str) 
     ->Result<File, std::io::Error> {
    let rfile = OpenOptions::new()
               .read(true)
               .open(file_name);
    rfile
}

fn read_file_to_string(f:&mut File) 
     -> Result<String, std::io::Error> {
  let mut contents = String::new();
  let bytes_rslt = f.read_to_string(&mut contents);
  if bytes_rslt.is_ok() {
    Ok(contents)
  }
  else {
      Err(Error::new(ErrorKind::Other, "read error"))
  }
}

#[derive(Debug)]
struct Basic {
    name: String,
    file: Option<File>,
    lines: usize,
}
impl Basic {
    fn new() -> Basic {
        Basic {
            name: String::new(),
            file: None,
            lines: 0,
        }
    }
    fn input(&mut self, name: &str) {
        self.name = name.to_string();
        let rslt = open_file_for_read(name);
        if let Ok(file) = rslt {
            self.file = Option::Some(file);
        }
        else {
            print!("\n  can't open file {:?}", name);
        }
    }
    fn compute(&mut self) {
        if let Some(file) = &mut self.file {
            let rslt = read_file_to_string(file);
            if let Ok(contents) = rslt {
                self.lines = 1;
                for ch in contents.chars() {
                    if ch == '\n' {
                        self.lines += 1;
                    }
                }
            }
        }
    }
    fn output(&self) {
        print!("\n  number of lines in {:?} = {}", self.name, self.lines);
    }
}
fn main() {

    print!("\n  -- counting lines in file --\n");

    let name = "./src/main.rs";

    let mut basic = Basic::new();
    basic.input(name);
    basic.compute();
    basic.output();

    println!("\n\n  That's all Folks!\n\n");
}
