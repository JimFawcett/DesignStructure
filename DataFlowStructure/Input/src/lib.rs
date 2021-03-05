/////////////////////////////////////////////////////////////
// DataFlowStructure::Input::lib.rs                        //
//   - Attempts to return line count from file             //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////
/*
  Note:
    - Input owns and instantiates Compute.
    - It attempts to open file and pass to Compute for
      processing.
    - Returns line count if successful
*/
use compute::*;
use std::fs::*;

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
    compute: Compute
}
impl Input {
    pub fn new() -> Input {
        Input {
            name: String::new(),
            compute: Compute::new()
        }
    }
    pub fn do_input(&mut self, name: &str) -> usize {
        let mut lines: usize = 0;
        self.name = name.to_string();
        let rslt = open_file_for_read(name);
        if let Ok(file) = rslt {
            self.compute.do_compute(name, file);
            lines = self.compute.lines();
        }
        else {
            print!("\n  can't open file {:?}", name);
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
