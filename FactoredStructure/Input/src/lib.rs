/////////////////////////////////////////////////////////////
// FactoredStructure::Input::lib.rs                        //
//   - Input attempts to open named file and return File   //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

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
}
impl Input {
    pub fn new() -> Input {
        Input {
            name: String::new(),
        }
    }
    pub fn do_input(&mut self, name: &str) -> Option<File> {
        let rslt = open_file_for_read(name);
        if let Ok(file) = rslt {
            return Some(file);
        }
        else {
            print!("\n  can't open file {:?}", name);
            return None;
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
