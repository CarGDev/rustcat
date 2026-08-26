use super::reader::process_lines;
use crate::config::CatConfig;
use crate::help::help;
use std::fs::File;
use std::io::{self, BufReader};

pub fn exec(config: CatConfig) {
    if config.show_help {
        help();
        return;
    }

    if config.filenames.is_empty() || config.is_stdin {
        let mut line_number = 1;
        let mut reader = Box::new(BufReader::new(io::stdin()));
        process_lines(&mut *reader, &config, &mut line_number);
    } else {
        for file_name in &config.filenames {
            match File::open(file_name) {
                Ok(file) => {
                    let mut line_number = 1;
                    let mut reader = BufReader::new(file);
                    process_lines(&mut reader, &config, &mut line_number);
                    println!(" ");
                }
                Err(e) => eprintln!("rustcat: {}: {}", file_name, e),
            }
        }
    }
}
