use std::fs::{read_dir, metadata};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn list_files_in_dir(dir_path: &str, tx: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let entries = read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            list_files_in_dir(&path.to_string_lossy(), tx.clone())?;
        } else {
            tx.send(path.to_string_lossy().to_string())?;
        }
    }
    Ok(())
}

fn worker(id: usize, rx: Receiver<String>, result_tx: Sender<usize>) {
    let mut total_size = 0;
    while let Ok(file_path) = rx.recv() {
        if let Ok(meta) = metadata(&file_path) {
            total_size += meta.len() as usize;
        }
    }
    result_tx.send(total_size).unwrap();
}

fn main() {
    let dir_path = "/path/to/directory"; // 替换为你要扫描的目录路径
    let num_workers = 4; // 工作线程的数量

    let (tx, rx) = channel();
    let (result_tx, mut result_rx) = channel();

    // 启动文件列表生成线程
    thread::spawn(move || {
        if let Err(e) = list_files_in_dir(dir_path, tx) {
            eprintln!("Error listing directory: {}", e);
        }
    });

    // 启动N个工作线程
    for i in 0..num_workers {
        let rx_clone = rx.clone();
        thread::spawn(move || worker(i, rx_clone, result_tx.clone()));
    }

    drop(tx); // 释放发送端，确保所有接收端能够结束循环

    let mut total_sizes = vec![0; num_workers];
    for _ in 0..num_workers {
        if let Ok(size) = result_rx.recv() {
            total_sizes.push(size);
        }
    }

    let grand_total: usize = total_sizes.iter().sum();
    println!("Total size of all files: {} bytes", grand_total);
}
