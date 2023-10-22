use std::{
    fs,
    io::{self, ErrorKind},
    process::Command,
};

use crate::{LBAct, LBErr, LBRes, META_FILE};

pub fn status() -> LBRes {
    let loc = match get_loc() {
        Err(e) => match e {
            LBErr::MetaMissing => return Ok(LBAct::Msg(e.to_string())),
            _ => return Err(e),
        },
        Ok((p, d)) => format!("'{}/{}'", p, d),
    };
    let rec = match get_rec() {
        Err(e) => match e {
            LBErr::NoRec => String::from("unknown"),
            _ => return Err(e),
        },
        Ok(c) => format!("'{}'", c),
    };
    Ok(LBAct::Msg(format!(
        "backup system for {}\nmost recent backup: {}",
        loc, rec
    )))
}

pub fn init(args: &[String]) -> LBRes {
    let loc = match args.get(0) {
        Some(l) => l,
        _ => return Err(LBErr::ArgMissing("file or directory to backup")),
    };
    match fs::write(META_FILE, format!("{}\n", loc)) {
        Ok(_) => Ok(LBAct::NoOp),
        Err(e) => Err(LBErr::IO(e)),
    }
}

pub fn load(args: &[String]) -> LBRes {
    let name = match args.get(0) {
        Some(n) => n.to_owned(),
        None => return Err(LBErr::ArgMissing("name of backup to load")),
    };
    let (path, dest) = get_loc()?;

    if !confirm(&format!(
        "load backup '{}' into '{}/{}'?\n(this will overwrite the current contents)",
        name, path, dest
    ))? {
        return Ok(LBAct::Msg(String::from("load cancelled")));
    }

    let mut rm_handle = match Command::new("rm")
        .args(["-r", &format!("{}/{}", path, dest)])
        .spawn()
    {
        Ok(h) => h,
        Err(e) => return Err(LBErr::IO(e)),
    };
    if let Err(e) = rm_handle.wait() {
        return Err(LBErr::IO(e));
    }

    let mut tar_handle = match Command::new("tar")
        .args(["-xf", &name, "-C", &path])
        .spawn()
    {
        Ok(h) => h,
        Err(e) => return Err(LBErr::IO(e)),
    };
    if let Err(e) = tar_handle.wait() {
        return Err(LBErr::IO(e));
    }
    update_meta(&name)?;
    Ok(LBAct::Msg(format!("loaded backup '{}'", name)))
}

pub fn new(args: &[String]) -> LBRes {
    let name = match args.get(0) {
        Some(n) => n.to_owned(),
        None => return Err(LBErr::ArgMissing("backup name")),
    };

    backup(&name)?;
    update_meta(&name)?;
    Ok(LBAct::Msg(format!("created new backup '{}'", name)))
}

pub fn update(args: &[String]) -> LBRes {
    let name = match args.get(0) {
        Some(f) => f.to_owned(),
        None => get_rec()?,
    };

    backup(&name)?;
    update_meta(&name)?;
    Ok(LBAct::Msg(format!("updated backup '{}'", name)))
}

fn update_meta(name: &str) -> Result<(), LBErr> {
    let loc = get_loc()?;
    let loc = format!("{}/{}", loc.0, loc.1);
    match fs::write(META_FILE, format!("{}\n{}\n", loc, name)) {
        Ok(_) => Ok(()),
        Err(e) => Err(LBErr::IO(e)),
    }
}

fn backup(name: &str) -> Result<(), LBErr> {
    let (path, source) = get_loc()?;
    let mut handle = match Command::new("tar")
        .args(["-czf", &name, "-C", &path, &source])
        .spawn()
    {
        Ok(h) => h,
        Err(e) => return Err(LBErr::IO(e)),
    };
    match handle.wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(LBErr::IO(e)),
    }
}

fn get_loc() -> Result<(String, String), LBErr> {
    let loc = match fs::read_to_string(META_FILE) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(LBErr::MetaMissing),
            _ => return Err(LBErr::MetaIO(e)),
        },
    };
    let mut loc = match loc.lines().next() {
        Some(l) => l.trim_end_matches('/').split('/').collect::<Vec<_>>(),
        None => return Err(LBErr::MetaCorrupt),
    };
    let dest = match loc.pop() {
        Some(s) => s.to_string(),
        None => return Err(LBErr::MetaCorrupt),
    };
    let path = if loc.is_empty() {
        String::from(".")
    } else {
        loc.join("/")
    };
    Ok((path, dest))
}

fn get_rec() -> Result<String, LBErr> {
    let rec = match fs::read_to_string(META_FILE) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(LBErr::MetaMissing),
            _ => return Err(LBErr::MetaIO(e)),
        },
    };
    match rec.lines().collect::<Vec<_>>().get(1) {
        Some(c) => Ok(c.to_string()),
        None => Err(LBErr::NoRec),
    }
}

fn confirm(msg: &str) -> Result<bool, LBErr> {
    println!("{} [y/n]", msg);
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(LBErr::IO(e));
    }
    Ok(input.to_lowercase().starts_with("y"))
}
