use taap;
pub fn main() {
    let mut argobject = taap::Argument::new("ls", "ls coreutil", "", "SpamixOfficial");
    argobject.add_option('l', "list", "1", None);
    let parsed_args = argobject.parse_args(None);
    dbg!(parsed_args);
}
