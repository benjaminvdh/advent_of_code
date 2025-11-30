use super::cave::{self, CavePath, RefCave};

pub struct CaveChecker;

impl cave::CaveChecker for CaveChecker {
    fn may_enter(&self, prefix: &CavePath, cave: &RefCave) -> bool {
        !cave.borrow().is_small() || !cave::is_cave_in_prefix(&cave, &prefix)
    }
}

pub fn print_success_message(num_paths: usize) {
    println!("There are {} paths through the caves.", num_paths);
}

pub fn get_number_of_paths(links: Vec<(String, String)>) -> usize {
    let checker = CaveChecker;

    crate::day_12::cave::get_number_of_paths(links, &checker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_paths() {
        let links = vec![
            (String::from("start"), String::from("A")),
            (String::from("start"), String::from("b")),
            (String::from("A"), String::from("c")),
            (String::from("A"), String::from("b")),
            (String::from("b"), String::from("d")),
            (String::from("A"), String::from("end")),
            (String::from("b"), String::from("end")),
        ];

        assert_eq!(get_number_of_paths(links), 10);
    }
}
