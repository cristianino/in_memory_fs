use std::collections::HashMap;
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

    // Método para agregar un archivo
    pub fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }

    // Método para agregar un directorio
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

    // Método para crear un archivo en el sistema de archivos
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

    // Método para crear un directorio en el sistema de archivos
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
}
