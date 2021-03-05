/////////////////////////////////////////////////////////////
// PlugInDataFlowStructure::Compute::test1.rs              //
//   - Attempts to read opened file to string, count lines //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use compute::*;
use input::Compute;
use compute::Output;
use std::fs::*;
use std::io::*;
// use std::option::*;

struct MockOutput {}
impl Output for MockOutput {
    fn do_output(&self, name: &str, lines: usize) {
        print!("\n  sending {} lines to {:?}", lines, name);
    }
}
impl MockOutput {
    pub fn new() -> MockOutput {
        MockOutput{}
    }
}

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
        let mut compute = ComputeImpl::new();
        let mo = Box::new(MockOutput::new());
        compute.set_output(mo);
        let _ = compute.do_compute(name, file);
    }
    print!("\n\n  That's all Folks!\n\n");
}