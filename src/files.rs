extern crate rand;
use self::rand::Rng;

use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::Read;
use std::sync::mpsc::Sender;

pub fn get_files(folder: &str, files: &mut Vec<String>) {
    if let Ok(paths) = read_dir(&Path::new(&folder)) {
        for path in paths {
            let file = get_path(path.unwrap().path());
            if is_src(&file){
                files.push(file.clone());
            }

            get_files(&file, files);
        }
    }
}

fn generate_random(n: u32) ->u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..n)
}

pub fn list_files(folder: String, txs: &Vec<Sender<String>>) {
    let len :u32 = txs.len() as u32;
    if let Ok(paths) = read_dir(&Path::new(&folder)) {
        for path in paths {
            let file = get_path(path.unwrap().path());
            if is_src(&file){
                let i = generate_random(len);
                let _ = txs.get(i as usize).expect(&format!("index {i} should be in range 0..{len}")).send(file);
            }else{
                list_files(file, txs);
            }
        }
    }
}

use std::process::Command;

/// A function to determine the file type of a given file path.
///
/// # Arguments
///
/// * `file_name` - A reference to the file name as a string.
///
/// # Returns
///
/// An `Option<String>` containing the file type if successful, or `None` otherwise.
pub fn get_file_type(file_name: &str) -> Option<String> {
    let mut command = Command::new("file");
    command.args([file_name]);

    match command.output() {
        Ok(output) => {
            let output_bytes = output.stdout;
            if let Ok(file_type) = String::from_utf8(output_bytes) {
                Some(file_type.lines().next()?.to_owned())
            } else {
                None
            }
        }
        Err(_) => None, // Error handling not shown for simplicity
    }
}

pub fn read_file(file_name: &str) -> Option<String> {
    let path = Path::new(&file_name);
    if let Ok(mut file) = File::open(&path) {
        let mut filecontent = String::new();
        match file.read_to_string(&mut filecontent) {
            Err(_) => None,
            Ok(_)  => Some(filecontent)
        }
    }else{
        None
    }
}

pub static EXTS: &'static [&'static str] = &[".rs", "hs",
                                             ".go", ".rb", ".rbw",
                                             ".java", ".scala", ".clj",
                                             ".js", ".cljs",
                                             ".cpp", ".c", ".h", ".m", ".mm",
                                             ".cs", ".fs", ".vb",
                                             ".py", ".pyc", ".pyd", ".pyo", ".pyw", ".pyz",
                                             ".php", ".phtml", ".php3", ".php4", ".php5", ".phps",
                                             ".pas",
                                             ".lisp", ".cl",".elisp",".el",
                                             ".vim",
                                             ".tcl", ".lua",
                                             ".pl", ".pm", ".t", ".pod"];

fn is_src(file: &str) -> bool {
    EXTS.iter()
    .filter(|x| file.ends_with(*x))
    .count() > 0
}

fn get_path(pb: PathBuf) -> String {
    let path = pb.as_path().to_str();
    match path {
        Some(s) => s,
        None    => ""
    }.to_string()
}
