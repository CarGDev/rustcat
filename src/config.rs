pub struct CatConfig {
    pub use_numbers: bool,
    pub use_no_blank_numbers: bool,
    pub filenames: Vec<String>,
    pub show_help: bool,
    pub show_char: bool,
    pub squeezing_blank_lines: bool,
    pub show_tabs: bool,
    pub show_nonprint: bool,
    pub is_stdin: bool,
}

impl Default for CatConfig {
    fn default() -> Self {
        CatConfig {
            is_stdin: false,
            use_numbers: false,
            use_no_blank_numbers: false,
            filenames: Vec::new(),
            show_help: false,
            show_char: false,
            squeezing_blank_lines: false,
            show_tabs: false,
            show_nonprint: false,
        }
    }
}
