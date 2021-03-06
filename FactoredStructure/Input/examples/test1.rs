/////////////////////////////////////////////////////////////
// FactoredStructure::Input::test1.rs                      //
//   - Input attempts to open named file and return File   //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use input::*;

fn main() {
    print!("\n  -- input::test1 --\n");

    let mut inp = Input::new();
    let name = "./src/lib.rs";
    let opt = inp.do_input(name);
    if let Some(_) = opt {
        print!("\n  opened file {:?}", name);
    }
    else {
        print!("\n  couldn't open file {:?}", name);
    }
    print!("\n\n  That's all Folks!\n\n");
}