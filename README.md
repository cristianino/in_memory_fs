
# In-Memory Filesystem

## Description

The In-Memory Filesystem is a Rust project designed to simulate a filesystem entirely in memory. This project aims to provide a better understanding of memory management, data structures, and filesystem APIs within the Rust programming language. It allows users to create, read, write, and delete files and directories without interacting with the actual disk storage.

## Features

- **Create Files and Directories:** Easily create files and directories in memory.
- **Read and Write Operations:** Perform read and write operations on in-memory files.
- **Delete Files and Directories:** Remove files and directories from the in-memory filesystem.
- **Serialization and Deserialization:** Serialize and deserialize the filesystem structure using `serde` and `serde_json`.

## Getting Started

To get started with the In-Memory Filesystem, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/your-username/in_memory_fs.git
cd in_memory_fs
cargo build
```

## Usage

Run the project with Cargo to start using the in-memory filesystem:

```bash
cargo run
```

### Example

Create UNIX directories and examples files
```rust
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
```
Print
```json
FileSystem {
    root: Directory {
        name: "/",
        files: {},
        directories: {
            "var": Directory {
                name: "var",
                files: {},
                directories: {
                    "www": Directory {
                        name: "www",
                        files: {},
                        directories: {
                            "html": Directory {
                                name: "html",
                                files: {
                                    "index.php": File {
                                        name: "index.php",
                                        content: [
                                            60,
                                            63,
                                            112,
                                            104,
                                            112,
                                            32,
                                            101,
                                            99,
                                            104,
                                            111,
                                            40,
                                            39,
                                            72,
                                            111,
                                            108,
                                            97,
                                            39,
                                            41,
                                            32,
                                            63,
                                            62,
                                        ],
                                    },
                                },
                                directories: {},
                            },
                        },
                    },
                },
            },
            "root": Directory {
                name: "root",
                files: {},
                directories: {},
            },
            "sys": Directory {
                name: "sys",
                files: {},
                directories: {},
            },
            "etc": Directory {
                name: "etc",
                files: {},
                directories: {
                    "init": Directory {
                        name: "init",
                        files: {},
                        directories: {},
                    },
                    "init.d": Directory {
                        name: "init.d",
                        files: {},
                        directories: {},
                    },
                },
            },
            "bin": Directory {
                name: "bin",
                files: {},
                directories: {},
            },
            "sbin": Directory {
                name: "sbin",
                files: {},
                directories: {},
            },
            "home": Directory {
                name: "home",
                files: {},
                directories: {
                    "cnino": Directory {
                        name: "cnino",
                        files: {
                            ".profile": File {
                                name: ".profile",
                                content: [
                                    101,
                                    120,
                                    112,
                                    111,
                                    114,
                                    116,
                                    61,
                                    39,
                                    36,
                                    80,
                                    65,
                                    84,
                                    72,
                                    61,
                                    36,
                                    80,
                                    65,
                                    84,
                                    72,
                                    39,
                                ],
                            },
                        },
                        directories: {},
                    },
                },
            },
            "usr": Directory {
                name: "usr",
                files: {},
                directories: {},
            },
            "dev": Directory {
                name: "dev",
                files: {},
                directories: {},
            },
            "boot": Directory {
                name: "boot",
                files: {},
                directories: {},
            },
            "media": Directory {
                name: "media",
                files: {},
                directories: {},
            },
            "opt": Directory {
                name: "opt",
                files: {},
                directories: {},
            },
            "tmp": Directory {
                name: "tmp",
                files: {},
                directories: {},
            },
        },
    },
}
```


## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your improvements.

## License

This project is licensed under the MIT License.