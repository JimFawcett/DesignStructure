// test1.rs

use compute::*;
use std::fs::*;
use std::io::*;

fn open_file_for_read(file_name:&str) 
     -> Result<File> {
    let rfile = OpenOptions::new()
               .read(true)
               .open(file_name);
    rfile
}

fn main() {
    print!("\n  -- compute::test1 --\n");

    let name = "./src/lib.rs";
    let rslt = open_file_for_read(name);
    if let Ok(file) = rslt {
        let mut compute = Compute::new();
        compute.do_compute(name, file);
    }
    print!("\n\n  That's all Folks!\n\n");
}