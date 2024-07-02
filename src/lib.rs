extern crate num_cpus;

mod files;
mod counting;

use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Mutex, Arc};

use crate::files::list_files;
pub use crate::counting::{Stats, Counter};
use crate::counting::{get_counters, get_stats};
use crate::counting::count_lines;

pub fn sloc(directory: String) -> (Vec<Counter>, Stats){
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
    return (counters, stats)
}
