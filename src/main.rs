extern crate clap;
extern crate num_cpus;
extern crate num_format;
extern crate sloc;

use clap::{Arg,App};
use num_format::{Locale, ToFormattedString};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Mutex, Arc};

// mod files;
// mod counting;

use std::cmp;

use sloc::files::list_files;
use sloc::counting::{Stats, Counter, get_counters, get_stats};
use sloc::counting::count_lines;

fn main() {
    println!("Source lines of code program...");
    let matches = App::new("Source lines of code")
        .version("1.0")
        .author("hejack0207 <hejack0207@sina.com>")
        .about("Source lines of codes program")
        .arg(Arg::with_name("summary")
             .short("s")
             .long("summary")
             .help("Display only summary one line")
             .takes_value(false))
        .arg(Arg::with_name("num")
             .short("n")
             .long("number")
             .help("Display only top number")
             .takes_value(true)
             .default_value("10"))
        .arg(Arg::with_name("exclude")
             .short("x")
             .long("exclude")
             .help("exclude following items, items are separated with commas")
             .takes_value(true))
        .arg(Arg::with_name("directory")
             .help("directory to stat")
             .index(1))
        .get_matches();

    let onlysummary = matches.is_present("summary");
    // println!("summary flag:{}",onlysummary);
    let mut directory = String::new();
    if let Some(ref dir) = matches.value_of("directory") {
        directory = dir.to_string();
        println!("directory:{}",dir);
    }

    let num_cores = num_cpus::get();
    println!("number of cores: {}", num_cores);

    let mut txs = Vec::new();
    let counters = Arc::new(Mutex::new(Vec::new()));
    let mut c = num_cores;
    while c > 0 {
        let ch = channel();
        txs.push(ch.0);
        // rxs.push(ch.1);
        let rx = ch.1;
        let counters = Arc::clone(&counters);
        thread::spawn(move ||{
            let mut counters = counters.lock().unwrap();
            let counter = count_lines(rx);
            counters.push(counter);
        });
        c -= 1;
    }

    let mut files: Vec<String> = Vec::new();
    thread::spawn(move ||{
        list_files(directory,&txs);
    });
    // get_files(directory, &mut files);
    let counters = get_counters(files);
    let stats = get_stats(&counters);

    if ! onlysummary {
        if let Some(num_str) = matches.value_of("num"){
            if let Ok(num) = num_str.parse::<usize>() {
                show_counters(&counters, num);
            }else{
                println!("invalid option value --num:{}",num_str);
            }
        }else{
            println!("invalid option value --num:{:?}",matches.value_of("num"));
        }
    }
    show_stats(&stats);
}

fn show_stats(stats: &Stats) {
    println!("Total files: {}", stats.files_count.to_formatted_string(&Locale::en));
    println!("Total loc: {}", stats.total_loc.to_formatted_string(&Locale::en));
    println!("Empty loc: {}", stats.empty_loc.to_formatted_string(&Locale::en));
}

fn show_counters(counters: &Vec<Counter>, num: usize) {
    let len = counters.len();

    // let max = if len < num { len } else { num };
    let min = cmp::min(len, num);

    if min > 0 {
        println!("{} biggest files:", min);
    }

    let mut i = 0;
    while i < min {
        println!("{position}. {total_loc} loc in {file_name}",
            position = (i + 1),
            total_loc = counters[i].total_loc,
            file_name = counters[i].file
            );
        i += 1;
    }
}
