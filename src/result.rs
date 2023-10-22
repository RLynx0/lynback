use std::fmt::Display;

#[derive(Debug)]
pub enum LBErr {
    CmdUnknown(String),
    ArgMissing(&'static str),
    MetaMissing,
    MetaCorrupt,
    MetaIO(std::io::Error),
    NoRec,
    IO(std::io::Error),
}
impl Display for LBErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LBErr::CmdUnknown(c) => write!(f, "unknown command: '{}'", c),
            LBErr::ArgMissing(a) => write!(f, "missing argument - specify {}", a),
            LBErr::MetaMissing => write!(f, "metadata missing - call lynback init"),
            LBErr::MetaCorrupt => write!(f, "metadata corrupt - call lynback init"),
            LBErr::MetaIO(e) => write!(f, "meatdata-io error: {}", e),
            LBErr::NoRec => write!(f, "no recent backup - create one with lynback new"),
            LBErr::IO(e) => write!(f, "io error: {}", e),
        }
    }
}
pub enum LBAct {
    NoOp,
    Msg(String),
}
pub type LBRes = Result<LBAct, LBErr>;
