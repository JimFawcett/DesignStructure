/////////////////////////////////////////////////////////////
// FactoredStructure::Executive::exec_proc_mod.rs          //
//   - Executive creates and uses all lower level parts    //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////
/*
  Extracted as Module since these could be useful in 
  other programs.
*/
/*-- extract command line arguments into vector --*/
pub fn parse_cmdln() -> Vec<String> {
    let cl_iter = std::env::args().into_iter();
    let args: Vec<String> = cl_iter.skip(1).collect();
    args
}
/*-- display command line args in comma seperated list --*/
pub fn show_cmdln(args: &Vec<String>)  {
    if args.len() == 0 {
        return;
    }
    print!("\n  {}", args[0]);
    for arg in &args[1..] {
        print!(", {}", arg);
    }
}
