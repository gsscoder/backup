# backup (bk)

Creates numbered backup copies of a file and allows restore.

## Install

**NOTE**: Rust 1.39 or higher is required.
```sh
# clone the repository
$ git clone https://github.com/gsscoder/backup.git

# change the working directory
$ cd backup

# build the executable
$ cargo build --release

# Copy it in a directory in your $PATH
$ sudo cp ./target/release/bk /opt/bin
```

## Usage

```sh
# Backup

$ bk Cargo.toml
$ bk Cargo.toml
$ bk Cargo.toml
$ bk Cargo.toml -v
bk: Cargo.toml: Backed up as Cargo.toml.bak.2
$ ll .
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml.bak
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml.bak.1
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml.bak.2


# Restore

$ bk -rv Cargo.toml.bak.1
k: Restore: Cargo.toml.bak.1 (y to confirm)? y
bk: Cargo.toml.bak.1: Restored as Cargo.toml
$ ll .
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml.bak
-rwxr-xr-x@  1 giacomo  staff   177B 11 Nov 08:57 Cargo.toml.bak.2
```

### Notes

- This command works only on files not directories.