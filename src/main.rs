mod lib_dir_file;
use lib_dir_file::FileSystem;

fn main() {
    let mut fs = FileSystem::new();

    // Create directories
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

    fs.create_file("/var/www/html", "index.php", b"<?php echo('Hola') ?>".to_vec());

    println!("{:#?}", fs);
}
