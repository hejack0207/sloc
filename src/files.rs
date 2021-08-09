use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::Read;

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

static EXTS: &'static [&'static str] = &[".rs", "hs",
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

#[test]
fn ext_length(){
    assert_eq!(42,EXTS.len());
}

