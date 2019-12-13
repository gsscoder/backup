
// Std
use std::vec::Vec;
use std::result::Result;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

// Third Party
use clap::{App, Arg};
use glob::glob;
use regex::Regex;

macro_rules! die {
    ($e:expr) => {
        println!("{}: {}", program_name(), $e);
        process::exit(1)
    }
}

fn explain(verbose: bool, message: &str) {
    if verbose {
        println!("{}: {}", program_name(), message);
    }
}

fn program_name() -> String {
    let exe = std::env::current_exe().unwrap();
    Path::new(exe.to_str().unwrap())
        .file_name().unwrap().to_str().unwrap().to_string()
}

fn bad_extension(path: &str) -> bool {
    let ext = Path::new(path).extension().unwrap().to_str().unwrap();
    ext.contains(" ")
}

fn is_backup(path: &str) -> bool {
    let re = Regex::new(r"\.bak(\.)*[0-9]*$").unwrap();
    re.is_match(path)
}

fn copy_file(from: &str, to: &str) -> io::Result<()> {
    match std::fs::copy(from, to) {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    }
}

fn try_fs<O,S>(operation: O, on_success: S, on_success_exit: bool)
    where O : Fn() -> io::Result<()>,
          S : Fn() -> () {
    match operation() {
        Ok(_) => {
            on_success();
            if on_success_exit {
                process::exit(0)
            }
        },
        Err(error) => {
            die!(error.to_string());
        }
    }
}

fn new_filename(filename: &str) -> String {   
   let pattern = format!("{}*.bak.*", filename);
   let paths: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
   let new_filename = if paths.len() == 0 {
           if Path::new(&format!("{}.bak", filename)).exists() {
               format!("{}.bak.1", filename)
           } else {
               format!("{}.bak", filename)
           }
       } else {
           let mut indexes = Vec::new();
           for path in paths {
               let re = Regex::new(r"\.bak(\.)*[0-9]*$").unwrap();
               let path_str = path.to_str().unwrap();
               if !bad_extension(path_str) {
                   let caps = re.captures(path_str).unwrap();
                   let index_text = caps.get(0).map_or("", |m| m.as_str());
                   let index = if index_text.len() - 4 == 0 {
                           1
                       } else {
                           index_text[5..index_text.len()].parse::<u32>().unwrap() + 1
                       };    
                   indexes.push(index);
               }
           }
           indexes.sort();
           indexes.reverse();
           format!("{}.bak.{}", filename, indexes[0].to_string())
       };
    return new_filename;
}

fn old_filename(filename: &str) -> String {
    let re = Regex::new(r"\.bak(\.)*[0-9]*$").unwrap();
    return re.replace(filename, "").to_string();
}

fn basename(path: &str) -> String {
    let path = Path::new(path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    if let Some(ext) = path.extension() {
        return format!("{}.{}", stem, ext.to_str().unwrap())
    }
    return stem.to_string()
}

fn confirm_restore(filename: &str) -> bool {
    print!("{}: Restore: {} (y to confirm)? ", program_name(), filename);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    if let Ok(_) = io::stdin().read_line(&mut line) {
        return line[0..1].to_lowercase() == "y".to_string()
    }
    return false;
}

fn main() {
    let matches = App::new("bk")
        .version("0.1.0")
        .about("Creates a backup copy of a file")
        .arg(
            Arg::with_name("FILE")
            .help("File to be backed up")
            .required(true)
            .index(1)
        )
        .arg(
            Arg::with_name("verbose")
            .short("v")
            .help(&format!("Causes {} to be verbose", program_name()))
        )
        .arg(
            Arg::with_name("restore")
            .short("r")
            .long("restore")
            .help(&format!("Restores a backup copy to its original name"))
        )        
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    if !Path::new(filename).exists() {
        die!(format!("{}: No such file or directory", filename));
    }
    let path = Path::new(filename);
    if path.is_dir() {
        die!(format!("{}: Is a directory", filename));
    }

    let verbose = matches.is_present("verbose");
    let backup = !matches.is_present("restore");

    if backup {
        // Create backup copy
        let bak_filename = new_filename(filename);
        try_fs(|| copy_file(filename, &bak_filename),
               || explain(verbose,
                    &format!("{}: Backed up as {}", basename(filename), bak_filename)), true)
    } else {
        // Restore backup copy
        if !is_backup(filename) {
            die!(format!("{}: Is not a backup file", filename).to_string());
        }
        if confirm_restore(filename) {
            let orig_filename = old_filename(filename);
            try_fs(|| std::fs::remove_file(Path::new(&orig_filename)),
                   || (), false);
            try_fs(|| copy_file(filename, &orig_filename),
                   || (), false);
            try_fs(|| std::fs::remove_file(Path::new(&filename)),
                   || explain(verbose, 
                        &format!("{}: Restored as {}", basename(filename), orig_filename)), true)
        }
    }
}