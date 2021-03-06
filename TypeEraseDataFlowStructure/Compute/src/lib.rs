/////////////////////////////////////////////////////////////
// TypeErasureDataFlowStructure::Compute::lib.rs           //
//   - Attempts to read opened file to string, count lines //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////
/* 
  Note:
    - creates instance of Output
    - attempts to read file to string and count its lines
    - sends results to Output
*/
use input::{Compute};
use std::fs::*;

pub trait Output {
    fn new() -> Self;
    fn do_output(&self, name: &str, lines: usize);
}

mod file_utilities;
use file_utilities::read_file_to_string;

#[derive(Debug)]
pub struct ComputeImpl<Out: Output> {
  lines: usize,
  out: Out
}
impl<Out:Output> Compute for ComputeImpl<Out> {
    fn new() -> ComputeImpl<Out> {
        ComputeImpl {
            lines: 0,
            out: Out::new()
        }
    }
    fn do_compute(&mut self, name: &str, mut file:File) {
        let rslt = read_file_to_string(&mut file);
        if let Ok(contents) = rslt {
            self.lines = 1;
            for ch in contents.chars() {
                if ch == '\n' {
                    self.lines += 1;
                }
            }
            self.out.do_output(name, self.lines);
        }
        else {
            print!("\n  could not read {:?}", name);
        }
    }
    fn lines(&self) -> usize {
        self.lines
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
