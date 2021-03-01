// FactoredStructure::compute

use std::fs::*;
use output::{Output};
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
  //file: Option<File>,
  lines: usize,
  out: Output
}
impl Compute {
    pub fn new() -> Compute {
        Compute {
            //file: Option::None,
            lines: 0,
            out: Output::new()
        }
    }
    pub fn do_compute(&mut self, name: &str, mut file:File) {
        // if let Some(file) = &mut file {
            let rslt = read_file_to_string(&mut file);
            if let Ok(contents) = rslt {
                self.lines = 1;
                for ch in contents.chars() {
                    if ch == '\n' {
                        self.lines += 1;
                    }
                }
            // }
            self.out.do_output(name, self.lines);
        }
        else {

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
