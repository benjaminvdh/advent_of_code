use std::env;
use std::path::PathBuf;

pub struct Args {
    pub quiet: bool,
    pub path: PathBuf,
    pub run_part_1: bool,
    pub run_part_2: bool,
}

pub fn parse() -> Args {
    let mut quiet = false;
    let mut path = None;
    let mut run_part_1 = true;
    let mut run_part_2 = true;

    for arg in env::args() {
        match arg.as_str() {
            "-q" | "--quiet" => quiet = true,
            "-1" | "--one" => run_part_2 = false,
            "-2" | "--two" => run_part_1 = false,
            p => path = Some(PathBuf::from(p)),
        }
    }

    Args {
        quiet,
        path: path.unwrap(),
        run_part_1,
        run_part_2,
    }
}
