pub use result::*;

mod funcs;
mod result;

const META_FILE: &str = ".lynback";

pub fn help() {
    println!(include_str!("../help.txt"));
}

pub fn run(com: &str, args: &[String]) -> LBRes {
    match com {
        "init" => funcs::init(args),
        "load" => funcs::load(args),
        "new" => funcs::new(args),
        "update" => funcs::update(args),
        "status" => funcs::status(),
        x => Err(LBErr::CmdUnknown(x.to_string())),
    }
}
