/////////////////////////////////////////////////////////////
// FactoredStructure::Compute::test1.rs                    //
//   - Compute attempts to read file and count lines       //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

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

fn test_compute(name: &str) -> usize {
    let mut lines = 0usize;
    let rslt = open_file_for_read(name);
    if let Ok(file) = rslt {
        let mut compute = Compute::new();
        compute.do_compute(name, file);
        lines = compute.lines();
    }
    else {
        print!("\n  couldn't open {:?}", name);
    }
    lines
}

fn main() {
    print!("\n  -- compute::test1 --\n");

    let name = "./src/lib.rs";
    let lines = test_compute(name);
    print!("\n  lines in {:?} = {}", name, lines);

    let name = "no-exist";
    let lines = test_compute(name);
    print!("\n  lines in {:?} = {}", name, lines);

    print!("\n\n  That's all Folks!\n\n");
}