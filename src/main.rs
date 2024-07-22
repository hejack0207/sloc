#[macro_use] extern crate debug_here;
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

use sloc::files::{get_files,list_files};
use sloc::counting::{Stats, Counter, get_counters, get_stats, add_stats};
use sloc::counting::count_lines;

fn main() {
    let matches = App::new("Source lines of code")
        .version("1.0")
        .author("hejack0207 <hejack0207@sina.com>")
        .about("Source lines of codes program")
        .arg(Arg::with_name("multithread")
             .short("m")
             .long("multithread")
             .help("enable multi thread mode")
             .takes_value(false))
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
        println!("directory: {}",dir);
    }

    let mut counters = vec![];
    if matches.is_present("multithread") {
        let num_cores = num_cpus::get();
        println!("number of cores: {}", num_cores);

        let mut txs = Vec::new();
        let counters_list = Arc::new(Mutex::new(Vec::new()));

        let mut c = num_cores;
        let mut handlers = Vec::new();
        while c > 0 {
            let ch = channel();
            txs.push(ch.0);
            let rx = ch.1;
            let counters_list = Arc::clone(&counters_list);
            let h = thread::spawn(move ||{
                let counters = count_lines(rx);
                let mut counters_list = counters_list.lock().unwrap();
                counters_list.push(counters);
            });
            handlers.push(h);
            c -= 1;
        }

        let h = thread::spawn(move ||{
            list_files(directory,&txs);
        });

        handlers.push(h);

        for h in handlers {
            let _ = h.join();
        }
        let counters_list = counters_list.lock().unwrap();
        counters = counters_list.concat();
    }else{
        println!("single thread mode");
        let mut files: Vec<String> = Vec::new();
        get_files(&directory, &mut files);
        counters = get_counters(files);
    }
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
