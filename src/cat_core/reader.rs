use super::non_print::evaluate;
use crate::config::CatConfig;
use std::io::BufRead;

pub fn process_lines(reader: &mut dyn BufRead, config: &CatConfig, line_number: &mut i32) {
    let mut raw = Vec::new();
    let mut blnk_lines = 0;
    loop {
        raw.clear();
        match reader.read_until(b'\n', &mut raw) {
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => {
                eprintln!("rustcat {e}");
                break;
            }
        };
        let mut line = if config.show_nonprint {
            evaluate(&raw)
        } else {
            String::from_utf8_lossy(&raw).to_string()
        };

        if config.show_tabs {
            line = line.replace('\t', "^I");
        }
        if config.squeezing_blank_lines {
            let mut blank = true;
            for ch in line.chars() {
                if ch != ' ' && ch != '\t' && ch != '\n' {
                    blank = false;
                    blnk_lines = 0;
                    break;
                }
            }
            if blank {
                blnk_lines += 1;
                if blnk_lines > 1 {
                    continue;
                }
            }
        }

        if config.use_no_blank_numbers {
            let mut blank = true;
            for ch in line.chars() {
                if ch != ' ' && ch != '\t' && ch != '\n' {
                    blank = false;
                    break;
                }
            }
            if blank {
                if config.show_char {
                    let trimmed = line.trim_end_matches('\n');
                    println!("{trimmed}$");
                } else {
                    let trimmed = line.trim_end();
                    println!("{trimmed}");
                }
            } else {
                if config.show_char {
                    let trimmed = line.trim_end_matches('\n');
                    println!("{line_number} | {trimmed}$");
                } else {
                    let trimmed = line.trim_end();
                    println!("{line_number} | {trimmed}");
                }
                *line_number += 1;
            }
        } else if config.use_numbers {
            if config.show_char {
                let trimmed = line.trim_end_matches('\n');
                println!("{line_number} | {trimmed}$");
            } else {
                let trimmed = line.trim_end();
                println!("{line_number} | {trimmed}");
            }
            *line_number += 1;
        } else {
            if config.show_char {
                let trimmed = line.trim_end_matches('\n');
                println!("{trimmed}$");
            } else {
                let trimmed = line.trim_end();
                println!("{trimmed}");
            }
        }
    }
}
