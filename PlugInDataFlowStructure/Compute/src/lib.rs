/////////////////////////////////////////////////////////////
// PlugInDataFlowStructure::Compute::lib.rs                //
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
use std::io::{Read, Error, ErrorKind};

pub trait Output {
    // fn new() -> Self;
    fn do_output(&self, name: &str, lines: usize);
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

pub struct ComputeImpl {
  lines: usize,
  out: Option<Box<dyn Output>>
}
impl Compute for ComputeImpl {
    fn new() -> ComputeImpl {
        ComputeImpl {
            lines: 0,
            out: None
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
            if let Some(plug) = &self.out {
                plug.do_output(name, self.lines);
            }
        }
        else {
            print!("\n  could not read {:?}", name);
        }
    }
    fn lines(&self) -> usize {
        self.lines
    }
}
impl ComputeImpl {
    pub fn set_output(&mut self, out: Box<dyn Output>) {
        &self.out.replace(out);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
