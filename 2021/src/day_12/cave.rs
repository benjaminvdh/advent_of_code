use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub trait CaveChecker {
    fn may_enter(&self, prefix: &CavePath, cave: &RefCave) -> bool;
}

#[derive(Debug)]
pub struct Cave {
    name: String,
    is_small: bool,
    links: Vec<Weak<RefCell<Cave>>>,
}

impl Cave {
    pub fn new(name: String) -> Rc<RefCell<Self>> {
        let is_small = name.chars().all(|c| c.is_lowercase());

        Rc::new(RefCell::new(Self {
            name,
            is_small,
            links: vec![],
        }))
    }

    pub fn is_small(&self) -> bool {
        self.is_small
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn get_number_of_paths<C: CaveChecker>(
    links: Vec<(String, String)>,
    cave_checker: &C,
) -> usize {
    let links = create_links(links);

    build_cave_map(&links)
        .map(|start| get_paths_from(vec![], start, cave_checker).len())
        .unwrap_or_default()
}

pub type RefCave = Rc<RefCell<Cave>>;

struct CaveLink {
    pub from: RefCave,
    pub to: RefCave,
}

fn create_links(links: Vec<(String, String)>) -> Vec<CaveLink> {
    let mut map = HashMap::<String, RefCave>::new();

    links
        .into_iter()
        .map(|(from, to)| CaveLink {
            from: map.entry(from.clone()).or_insert(Cave::new(from)).clone(),
            to: map.entry(to.clone()).or_insert(Cave::new(to)).clone(),
        })
        .collect()
}

fn build_cave_map(links: &[CaveLink]) -> Option<RefCave> {
    for link in links {
        connect(&link.from, &link.to);
    }

    find_starting_cave(links)
}

fn find_starting_cave(links: &[CaveLink]) -> Option<RefCave> {
    links
        .iter()
        .find(|link| link.from.borrow().name == "start")
        .map(|link| link.from.clone())
}

fn connect(a: &RefCave, b: &RefCave) {
    a.borrow_mut().links.push(Rc::downgrade(b));
    b.borrow_mut().links.push(Rc::downgrade(a));
}

pub type CavePath = Vec<Weak<RefCell<Cave>>>;

fn get_paths_from<C: CaveChecker>(
    mut prefix: CavePath,
    cave: RefCave,
    cave_checker: &C,
) -> Vec<CavePath> {
    if cave.borrow().name == "end" {
        return vec![vec![Rc::downgrade(&cave)]];
    }

    prefix.push(Rc::downgrade(&cave));

    let mut paths = vec![];

    for link in &cave.borrow().links {
        if let Some(link) = link.upgrade() {
            if cave_checker.may_enter(&prefix, &link) {
                for mut path in get_paths_from(prefix.clone(), link, cave_checker).into_iter() {
                    path.push(Rc::downgrade(&cave));
                    paths.push(path);
                }
            }
        }
    }

    paths
}

pub fn is_cave_in_prefix(cave: &RefCave, prefix: &[Weak<RefCell<Cave>>]) -> bool {
    prefix.iter().any(|parent| {
        parent
            .upgrade()
            .map(|parent| parent.borrow().name == cave.borrow().name)
            .unwrap_or(false)
    })
}
