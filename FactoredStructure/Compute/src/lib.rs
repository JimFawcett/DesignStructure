/////////////////////////////////////////////////////////////
// FactoredStructure::Compute::lib.rs                      //
//   - Input attempts to read File to string & count lines //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use std::fs::*;
use std::io::{Read, Error, ErrorKind};

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
pub struct Compute {
  lines: usize,
}
impl Compute {
    pub fn new() -> Compute {
        Compute {
            lines: 0,
        }
    }
    /*-- read file, count lines and save count --*/
    pub fn do_compute(&mut self, name: &str, mut file:File) {
        let rslt = read_file_to_string(&mut file);
        if let Ok(contents) = rslt {
            self.lines = 1;
            for ch in contents.chars() {
                if ch == '\n' {
                    self.lines += 1;
                }
            }
        }
        else {
            print!("\n  couldn't open {:?}", name);
        }
    }
    /*-- return saved line count --*/
    pub fn lines(&self) -> usize {
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
