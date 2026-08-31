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

// In this case, all the default values are also the default value of that type,
// so this whole impl block could be replaced with `#[derive(Default)]`.
// However, if you wanted to change these defaults or add a new one that is not the type's default,
// (for instance, if you changed `use_no_blank_numbers: false` to `use_blank_numbers: true`)
// then you would need a full impl like this.
// This also makes explicit that you intended these to be the default values,
// so there is a case to be made that spelling it out like this is better anyway.
// As such, I leave this as-is, but note that for many other types, `#[derive(Default)]` is preferred.
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
