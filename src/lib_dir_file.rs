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
            root: Directory::new("/"),
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

    // Método para guardar el estado del sistema de archivos en un archivo
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = StdFile::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    // Método para cargar el estado del sistema de archivos desde un archivo
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = StdFile::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: FileSystem = serde_json::from_str(&contents).unwrap();
        Ok(deserialized)
    }
}
