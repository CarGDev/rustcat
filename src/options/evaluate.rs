use super::flags::valid_flags;
use crate::config::CatConfig;

pub fn read() -> CatConfig {
    let args: Vec<String> = std::env::args().collect();
    let mut config = CatConfig::default();
    let len = args.len();

    let flags = valid_flags();
    let mut has_valid_flag = std::env::args().any(|arg| flags.contains(&arg.as_str()));
    let get_flags: Vec<String> = std::env::args()
        .filter(|arg| arg.starts_with('-') && !arg.starts_with("--"))
        .collect();

    for mut flag in get_flags {
        flag = flag.replace('-', "");
        for ch in flag.chars() {
            has_valid_flag = flags.contains(&ch.to_string().as_str());
        }
    }

    if !has_valid_flag {
        eprintln!("rustcat: invalid flag. Use -h of --help for help.\n");
        std::process::exit(2);
    }

    for i in 1..len {
        let arg = &args[i];
        if arg.starts_with('-') {
            let str_len = args[i].len();
            if str_len > 2 && !arg.contains("--") {
                for ch in args[i].chars() {
                    if ch == '-' {
                        continue;
                    }
                    match ch {
                        'h' => config.show_help = true,
                        'v' => config.show_nonprint = true,
                        'A' => {
                            config.show_char = true;
                            config.show_tabs = true;
                            config.show_nonprint = true;
                        }
                        'T' => config.show_tabs = true,
                        'n' => config.use_numbers = true,
                        's' => config.squeezing_blank_lines = true,
                        'b' => {
                            config.use_no_blank_numbers = true;
                            config.use_numbers = false;
                        }
                        'E' => config.show_char = true,
                        _ => {
                            eprintln!("rustcat: invalid flag. Use -h of --help for help.\n");
                            std::process::exit(2);
                        }
                    }
                }
                continue;
            }
            match args[i].as_str() {
                "-" => config.is_stdin = true,
                "-h" | "--help" => config.show_help = true,
                "-v" => config.show_nonprint = true,
                "-A" | "--show-all" => {
                    config.show_char = true;
                    config.show_tabs = true;
                    config.show_nonprint = true;
                }
                "-T" => config.show_tabs = true,
                "-n" | "--number" => config.use_numbers = true,
                "-s" => config.squeezing_blank_lines = true,
                "-b" => {
                    config.use_no_blank_numbers = true;
                    config.use_numbers = false;
                }
                "-E" => config.show_char = true,
                &_ => {
                    eprintln!(
                        "rustcat: invalid flag. Use -h or --help for help. {}\n",
                        args[i].as_str()
                    );
                    std::process::exit(2);
                }
            }
        } else if !config.is_stdin {
            config.filenames.push(args[i].clone());
        }
    }

    config
}
