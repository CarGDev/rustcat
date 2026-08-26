pub fn help() {
    println!("Usage: rustcat [OPTION]... [FILE]...");
    println!("Concatenate FILE(s) to standard output.");
    println!("With no FILE, or when FILE is -, read standard input.\n");
    println!("Options:\n");
    println!("-b              number only non-blank output lines (overrides -n)");
    println!("-E              show $ at end of each line");
    println!("-n, --number    number all output lines");
    println!("-s              squeeze consecutive blank lines into one");
    println!("-T              show tabs as ^I");
    println!("-v              show non-printing characters (^X, M-x)");
    println!("-A              equivalent of -vET");
    println!("-h, --help      show this help and exit\n");
    println!("Examples:\n");
    println!("ccat notes.txt            print notes.txt");
    println!("ccat -n notes.txt         print notes.txt with line numbers");
    println!("ls | ccat                 print whatever comes through the pipe");
}
