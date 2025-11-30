use super::cave::{self, CavePath, RefCave};

pub struct CaveChecker;

impl cave::CaveChecker for CaveChecker {
    fn may_enter(&self, prefix: &CavePath, cave: &RefCave) -> bool {
        !cave.borrow().is_small()
            || !cave::is_cave_in_prefix(&cave, &prefix)
            || (!is_start_or_end(&cave) && !visited_small_cave_twice(prefix))
    }
}

pub fn print_success_message(num_paths: usize) {
    println!(
        "There are {} paths through the caves, possibly visiting a single small one twice.",
        num_paths
    );
}

pub fn get_number_of_paths(links: Vec<(String, String)>) -> usize {
    let checker = CaveChecker;

    crate::day_12::cave::get_number_of_paths(links, &checker)
}

fn is_start_or_end(cave: &RefCave) -> bool {
    cave.borrow().name() == "start" || cave.borrow().name() == "end"
}

fn visited_small_cave_twice(prefix: &CavePath) -> bool {
    for (i, cave) in prefix.iter().enumerate() {
        if let Some(cave) = cave.upgrade() {
            if cave.borrow().is_small() && cave::is_cave_in_prefix(&cave, &prefix[i + 1..]) {
                return true;
            }
        }
    }

    false
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

        assert_eq!(get_number_of_paths(links), 36);
    }
}
