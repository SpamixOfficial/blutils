use std::fs;
use taap;

pub fn main() {
    let mut argobject = taap::Argument::new("cat", "cat coreutil", "", "SpamixOfficial");
    argobject.add_arg("FILE", "+", )
    let parsed_args = argobject.parse_args(None); 
    dbg!(parsed_args); 
}
