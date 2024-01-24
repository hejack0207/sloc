extern crate clap;
use clap::{Arg,App};
extern crate num_format;
use num_format::{Locale, ToFormattedString};

mod files;
mod counting;

use std::cmp;

use files::get_files;
use counting::{Stats, Counter, get_counters, get_stats};

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

    let onlysummary = matches.is_present("summary"); //matches.value_of("summary").unwrap_or(false);
    println!("summary flag:{}",onlysummary);
    let mut directory = ".";
    if let Some(ref dir) = matches.value_of("directory") {
        directory = dir;
        println!("directory:{}",dir);
    }

    let mut files: Vec<String> = Vec::new();
    get_files(directory, &mut files);
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
    let max = cmp::max(len, num);

    if max > 0 {
        println!("{} biggest files:", max);
    }

    let mut i = 0;
    while i < max {
        println!("{position}. {total_loc} loc in {file_name}",
            position = (i + 1),
            total_loc = counters[i].total_loc,
            file_name = counters[i].file
            );
        i += 1;
    }
}
