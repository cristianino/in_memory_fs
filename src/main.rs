mod lib_dir_file;

fn main() {
    let filename = "filesystem.json";
    let fs = filesystem_service::initialize_filesystem(filename);

    println!("{:#?}", fs);
}

mod filesystem_service {
    use std::path::Path;
    use crate::lib_dir_file::FileSystem;

    pub fn initialize_filesystem(filename: &str) -> FileSystem {
        if Path::new(filename).exists() {
            FileSystem::load_from_file(filename).expect("Failed to load filesystem")
        } else {
            let mut fs = FileSystem::new();

            fs.create_directory("/", "root");

            fs.create_directory("/", "home");
            fs.create_directory("/home", "cnino");

            fs.create_directory("/", "boot");

            fs.create_directory("/", "bin");

            fs.create_directory("/", "sbin");

            fs.create_directory("/", "sys");

            fs.create_directory("/", "etc");
            fs.create_directory("/etc", "init");
            fs.create_directory("/etc", "init.d");

            fs.create_directory("/", "usr");
            fs.create_directory("/", "dev");
            fs.create_directory("/", "media");
            fs.create_directory("/", "opt");
            fs.create_directory("/", "tmp");
            fs.create_directory("/", "var");
            fs.create_directory("/var", "www");
            fs.create_directory("/var/www", "html");

            // Create files
            fs.create_file("/home/cnino", ".profile", b"export='$PATH=$PATH'".to_vec());

            fs.create_file(
                "/var/www/html",
                "index.php",
                b"<?php echo('Hola') ?>".to_vec(),
            );

            fs.save_to_file(filename).expect("Failed to save filesystem");
            fs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::filesystem_service;
    use std::path::Path;
    use std::fs;

    fn setup(test_filename: &str) {
        if Path::new(test_filename).exists() {
            fs::remove_file(test_filename).expect("Failed to delete existing test file");
        }
    }

    fn cleanup(test_filename: &str) {
        if Path::new(test_filename).exists() {
            fs::remove_file(test_filename).expect("Failed to delete test file");
        }
    }

    #[test]
    fn test_initialize_filesystem_creates_new() {
        let test_filename = "test_initialize_filesystem_creates_new.json";
        setup(test_filename);

        let fs = filesystem_service::initialize_filesystem(test_filename);

        // Verificar que el archivo se creó
        assert!(Path::new(test_filename).exists(), "File was not created");

        // Verificar el contenido del sistema de archivos
        assert!(fs.root.directories.contains_key("home"));
        assert!(fs.root.directories.get("home").unwrap().directories.contains_key("cnino"));

        cleanup(test_filename);
    }

    #[test]
    fn test_initialize_filesystem_loads_existing() {
        let test_filename = "test_initialize_filesystem_loads_existing.json";
        setup(test_filename);

        // Create a filesystem and save it
        {
            let mut fs = filesystem_service::initialize_filesystem(test_filename);
            fs.create_file("/home/cnino", "testfile.txt", b"test content".to_vec());
            fs.save_to_file(test_filename).expect("Failed to save filesystem");
        }

        // Verificar que el archivo se creó
        assert!(Path::new(test_filename).exists(), "File was not created");

        // Load the filesystem and check the contents
        let fs = filesystem_service::initialize_filesystem(test_filename);
        let home_dir = fs.root.directories.get("home").unwrap();
        let cnino_dir = home_dir.directories.get("cnino").unwrap();
        let test_file = cnino_dir.files.get("testfile.txt").unwrap();

        assert_eq!(test_file.name, "testfile.txt");
        assert_eq!(test_file.content, b"test content");

        cleanup(test_filename);
    }
}
