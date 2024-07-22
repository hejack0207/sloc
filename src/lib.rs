extern crate num_cpus;

mod files;
mod counting;

use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Mutex, Arc};

use crate::files::{list_files,get_files};
pub use crate::counting::{Stats, Counter};
use crate::counting::{get_counters, get_stats};
use crate::counting::count_lines;

pub fn sloc(directory: String, mt: bool) -> (Vec<Counter>, Stats){
    if mt {
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

        let counters = counters_list.concat();
        let stats = get_stats(&counters);
        return (counters, stats)
    }else{
        println!("single thread mode");
        let mut files: Vec<String> = Vec::new();
        get_files(&directory, &mut files);

        let counters = get_counters(&files);
        let stats = get_stats(&counters);
        return (counters, stats)
    }
}
