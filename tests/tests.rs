extern crate sloc;

extern crate clap;
use clap::{Arg,App};

#[test]
fn parse_opt() {
    let m = App::new("myprog")
    .arg(Arg::with_name("summary")
        .short("s"))
    .get_matches_from(vec![
        "myprog2", "-s"
    ]);

    assert!(m.is_present("summary"));
}

#[test]
fn get_files_test() {
    let mut files: Vec<String> = Vec::new();
    sloc::files::get_files("./test_data/", &mut files);
    assert_eq!(2, files.len());
}

#[test]
fn get_counters_test() {
    let mut files: Vec<String> = Vec::new();
    sloc::files::get_files("./test_data/", &mut files);
    let counters = sloc::counting::get_counters(files);
    assert_eq!(2, counters.len());
}

#[test]
fn get_stats_test(){
    let mut files: Vec<String> = Vec::new();
    sloc::files::get_files("./test_data/", &mut files); 
    let counters = sloc::counting::get_counters(files);
    let stats = sloc::counting::get_stats(&counters);

    assert_eq!(2, stats.files_count);
    assert_eq!(10, stats.total_loc);
    assert_eq!(12, stats.empty_loc);
}

#[test]
fn ext_length(){
    assert_eq!(42,sloc::files::EXTS.len());
}


