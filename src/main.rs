extern crate chrono;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io;
use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let filename = "log-".to_string() + &utc.to_string() + &".txt".to_string();
    let f = File::create(filename).expect("Unable to open file");
    let mut f = BufWriter::new(f);
    loop {
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("Unable to read.");
        f.write(data.as_bytes()).expect("Unable to write data");
        f.flush().expect("Unable to finished.");
    }
}
