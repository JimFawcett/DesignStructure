// FactoredStructure::input::lib.rs

use compute::*;
use std::fs::*;
// use std::io::{Read, Error, ErrorKind};

fn open_file_for_read(file_name:&str) 
     ->Result<File, std::io::Error> {
    let rfile = OpenOptions::new()
               .read(true)
               .open(file_name);
    rfile
}

#[derive(Debug)]
pub struct Input {
    name: String,
    // file: Option<File>,
    compute: Compute
}
impl Input {
    pub fn new() -> Input {
        Input {
            name: String::new(),
            // file: Option::<File>::None,
            compute: Compute::new()
        }
    }
    pub fn do_input(&mut self, name: &str) {
        self.name = name.to_string();
        let rslt = open_file_for_read(name);
        if let Ok(file) = rslt {
            // self.file = Option::Some(file);
            self.compute.do_compute(name, file);
        }
        else {
            print!("\n  can't open file {:?}", name);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
