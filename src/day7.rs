use itertools::Itertools;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct DirEntry {
    pub name: String,
    pub parent: Option<Rc<DirEntry>>,
    pub subdirs: HashMap<String, Rc<DirEntry>>,
    pub files: Vec<(String, usize)>,
}

impl DirEntry {
    pub fn new(name: String, parent: Option<Rc<DirEntry>>) -> DirEntry {
        DirEntry {
            name,
            parent,
            subdirs: HashMap::new(),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, name: String, size: usize) {
        self.files.push((name, size));
    }

    pub fn add_subdir(&mut self, name: String) {
        let dir = Rc::new(DirEntry::new(name.clone(), Some(Rc::new(self.clone()))));
        self.subdirs.insert(name, dir);
    }

    pub fn cd(&self, dir: String) -> Option<&DirEntry> {
        if dir == "/" {
            return self.root();
        }
        if let Some(found) = self.subdirs.get(&dir) {
            return Some(found);
        }
        None
    }

    pub fn root(&self) -> Option<&DirEntry> {
        let mut current_entry = self;
        while let Some(parent) = current_entry.parent.as_ref() {
            current_entry = parent.as_ref();
        }
        Some(current_entry)
    }
}

fn parse(str: &str) -> Option<DirEntry> {
    let parsed = str
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .collect_vec();
    None
}

pub fn solve(str: &str) -> (usize, usize) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file() {
        let mut dir = DirEntry::new("root".to_string(), None);
        assert_eq!(dir.files.len(), 0);
        dir.add_file("file1".to_string(), 100);
        assert_eq!(dir.files.len(), 1);
        assert_eq!(dir.files[0].0, "file1".to_string());
        assert_eq!(dir.files[0].1, 100);
    }

    #[test]
    fn test_add_subdir() {
        let mut dir = DirEntry::new("root".to_string(), None);
        assert_eq!(dir.subdirs.len(), 0);
        dir.add_subdir("subdir1".to_string());
        assert_eq!(dir.subdirs.len(), 1);
        assert!(dir.subdirs.contains_key("subdir1"));
    }

    #[test]
    fn test_cd() {
        let mut dir = DirEntry::new("root".to_string(), None);
        assert_eq!(dir.name, "root".to_string());
        dir.add_subdir("subdir1".to_string());
        let subdir1 = dir.subdirs["subdir1"].clone();
        assert_eq!(subdir1.name, "subdir1".to_string());
        assert_eq!(subdir1.parent.as_ref().unwrap().name, "root".to_string());
        assert_eq!(
            dir.cd("subdir1".to_string()).unwrap().name,
            "subdir1".to_string()
        );
        assert_eq!(dir.cd("/".to_string()).unwrap().name, "root".to_string());
        assert_eq!(dir.cd("nonexistent".to_string()).is_none(), true);
    }

    #[test]
    fn test_root() {
        let mut dir = DirEntry::new("root".to_string(), None);
        assert_eq!(dir.name, "root".to_string());
        assert_eq!(dir.parent.is_none(), true);
        dir.add_subdir("subdir1".to_string());
        let mut subdir1 = dir.subdirs["subdir1"].clone();
        assert_eq!(subdir1.name, "subdir1".to_string());
        assert_eq!(subdir1.parent.as_ref().unwrap().name, "root".to_string());
        assert_eq!(subdir1.root().unwrap().name, "root".to_string());
        assert_eq!(dir.root().unwrap().name, "root".to_string());

        let mut subdir1 = Rc::make_mut(&mut subdir1);
        subdir1.add_subdir("subdir2".to_string());
        let subdir2 = subdir1.subdirs["subdir2"].clone();
        assert_eq!(subdir2.name, "subdir2".to_string());
        assert_eq!(subdir2.parent.as_ref().unwrap().name, "subdir1".to_string());
        assert_eq!(subdir2.root().unwrap().name, "root".to_string());
    }
}
