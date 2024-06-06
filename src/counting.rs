use files::{read_file};

pub struct Counter{
    pub file: String,
    pub file_type: String,
    pub total_loc: u64,
    pub empty_loc: u64
}

pub struct Stats{
    pub files_count: u64,
    pub total_loc: u64,
    pub empty_loc: u64
}

pub fn get_counters(files: Vec<String>) -> Vec<Counter> {
    let mut counters: Vec<_> = files.iter()
        .map(|x| get_file_loc(&x))
        // .filter(|x| match *x {
        //     Some(_) => true,
        //     None    => false
        // })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    counters.sort_by(|a, b| b.total_loc.cmp(&a.total_loc));

    counters
}

pub fn get_stats(counters: &Vec<Counter>) -> Stats {
    let mut stats = Stats{
        files_count: 0,
        total_loc: 0,
        empty_loc: 0
    };

    for counter in counters {
        stats.files_count += 1;
        stats.total_loc += counter.total_loc;
        stats.empty_loc += counter.empty_loc;
    }

    stats
}

fn is_new_line_char(c: char) -> bool {
    c == '\n'
}

fn is_empty_char(c: char) -> bool {
    match c {
        ' '  => true,
        '\t' => true,
        _    => false
    }
}

fn get_file_loc(file_name: &str) -> Option<Counter> {
    let src_txt = read_file(file_name);
    match src_txt {
        Some(s) => Some(get_loc(file_name.to_string(), s)),
        None    => None
    }
}

fn get_loc(file: String, src_txt: String) -> Counter {
    let mut total_loc = 0;
    let mut empty_loc = 0;
    let v: Vec<char> = src_txt.chars().collect();
    let len = v.len();
    let mut empty = true;

    let mut i = 0;
    while i < len {
        match v[i] {
            c if is_new_line_char(c) && empty  => {
                empty_loc += 1;
                empty = true;
            },
            c if is_new_line_char(c) && !empty => {
                total_loc += 1;
                empty = true;
            },
            c if !is_empty_char(c) => empty = false,
            _                      => ()
        };

        i += 1;
    }
    if (len > 0) && empty {
        empty_loc += 1;
    }

    Counter {
        file: file,
        file_type: "".to_string(),
        total_loc: total_loc,
        empty_loc: empty_loc
    }
}
