mod lib_dir_file;
use lib_dir_file::FileSystem;

fn main() {
    let mut fs = FileSystem::new();

    // Crear directorios
    fs.create_directory("/", "dir1");
    fs.create_directory("/dir1", "dir2");

    // Crear archivos
    fs.create_file("/dir1/dir2", "file1.txt", b"Hello, world!".to_vec());

    println!("{:#?}", fs);
}
