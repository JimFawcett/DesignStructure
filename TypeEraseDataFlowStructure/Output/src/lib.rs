/////////////////////////////////////////////////////////////
// TypeErasureDataFlowStructure::Output::lib.rs            //
//   - Sends results to console                            //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use compute::Output;
#[derive(Debug)]
pub struct OutputImpl {

}
impl Output for OutputImpl {
    fn new() -> OutputImpl {
        OutputImpl {}
    }
    fn do_output(&self, name: &str, lines: usize) {
        print!("\n  file {:?} has {} lines of code", name, lines);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
