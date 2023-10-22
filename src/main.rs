use lynback::{LBAct, LBRes};
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    match args.get(1) {
        None => lynback::help(),
        Some(c) => resolve(lynback::run(c, &args[2..])),
    }
}

fn resolve(res: LBRes) {
    match res {
        Ok(LBAct::Msg(s)) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        _ => (),
    }
}
