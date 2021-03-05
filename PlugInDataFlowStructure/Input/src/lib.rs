/////////////////////////////////////////////////////////////
// PlugInDataFlowStructure::Input::lib.rs                  //
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
// use compute::*;
use std::fs::*;

pub trait Compute {
    fn new() -> Self;
    fn do_compute(&mut self, name: &str, file:File);
    fn lines(&self) -> usize;
}

fn open_file_for_read(file_name:&str) 
     ->Result<File, std::io::Error> {
    let rfile = OpenOptions::new()
               .read(true)
               .open(file_name);
    rfile
}

#[derive(Debug)]
pub struct Input<T: Compute> {
    name: String,
    compute: T
}
impl<T: Compute> Input<T> {
    pub fn new() -> Input<T> {
        Input {
            name: String::new(),
            compute: T::new()
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
    pub fn get_app(&mut self) -> &mut T {
        &mut self.compute
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
