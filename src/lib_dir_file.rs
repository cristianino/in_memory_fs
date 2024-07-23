use std::collections::HashMap;
use std::fs::File as StdFile;
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    pub name: String,
    pub files: HashMap<String, File>,
    pub directories: HashMap<String, Directory>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }

    pub fn add_directory(&mut self, dir: Directory) {
        self.directories.insert(dir.name.clone(), dir);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystem {
    pub root: Directory,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Directory::new("root"),
        }
    }

    pub fn create_file(&mut self, path: &str, name: &str, content: Vec<u8>) {
        let parts = path.split('/').collect::<Vec<&str>>();
        let file = File {
            name: name.to_string(),
            content,
        };
        let mut current_dir = &mut self.root;
        for part in parts.iter() {
            if *part != "" {
                current_dir = current_dir.directories.get_mut(*part).expect("Directory not found");
            }
        }
        current_dir.add_file(file);
    }

    pub fn create_directory(&mut self, path: &str, name: &str) {
        let parts = path.split('/').collect::<Vec<&str>>();
        let dir = Directory::new(name);
        let mut current_dir = &mut self.root;
        for part in parts.iter() {
            if *part != "" {
                current_dir = current_dir.directories.get_mut(*part).expect("Directory not found");
            }
        }
        current_dir.add_directory(dir);
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = StdFile::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = StdFile::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: FileSystem = serde_json::from_str(&contents).unwrap();
        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup(test_filename: &str) {
        if std::path::Path::new(test_filename).exists() {
            fs::remove_file(test_filename).expect("Failed to delete existing test file");
        }
    }

    fn cleanup(test_filename: &str) {
        if std::path::Path::new(test_filename).exists() {
            fs::remove_file(test_filename).expect("Failed to delete test file");
        }
    }

    #[test]
    fn test_create_file() {
        let mut fs = FileSystem::new();
        fs.create_directory("/", "dir1");
        fs.create_file("/dir1", "file1.txt", b"Hello, world!".to_vec());

        let dir = fs.root.directories.get("dir1").expect("Directory not found");
        let file = dir.files.get("file1.txt").expect("File not found");

        assert_eq!(file.name, "file1.txt");
        assert_eq!(file.content, b"Hello, world!");
    }

    #[test]
    fn test_create_directory() {
        let mut fs = FileSystem::new();
        fs.create_directory("/", "dir1");
        fs.create_directory("/dir1", "dir2");

        let dir1 = fs.root.directories.get("dir1").expect("Directory not found");
        let dir2 = dir1.directories.get("dir2").expect("Directory not found");

        assert_eq!(dir1.name, "dir1");
        assert_eq!(dir2.name, "dir2");
    }

    #[test]
    fn test_save_and_load_filesystem() {
        let test_filename = "test_save_and_load_filesystem.json";
        setup(test_filename);

        let mut fs = FileSystem::new();
        fs.create_directory("/", "dir1");
        fs.create_file("/dir1", "file1.txt", b"Hello, world!".to_vec());

        fs.save_to_file(test_filename).expect("Failed to save filesystem");

        let loaded_fs = FileSystem::load_from_file(test_filename).expect("Failed to load filesystem");

        let dir = loaded_fs.root.directories.get("dir1").expect("Directory not found");
        let file = dir.files.get("file1.txt").expect("File not found");

        assert_eq!(file.name, "file1.txt");
        assert_eq!(file.content, b"Hello, world!");

        cleanup(test_filename);
    }
}
