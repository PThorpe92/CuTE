use crate::curl::Flag;

pub struct Wget {
    pub cmd: &'static str,
    pub opts: Vec<Flag>,
}
