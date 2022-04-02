use std::path::PathBuf;
use std::env;

const HELP: &'static str = "
USAGE:
    leuchtkraft [FLAGS] [file-name]

FLAGS:
    -h, --help          Prints help information
    -i, --interactive   Enter a Repl once the program exits
    -nc, --no-color     Disable colored diagnostics
    -v, --version       Prints version information

ARGS:
    <file-name>         Path to an executable Leuchtkraft script
";

fn info() -> String {
    format!("Leuchtkraft version {}\nMaintained by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"))
}

pub struct Options {
    /// Path to a executable Leuchtkraft script
    pub file_name: Option<PathBuf>,

    /// Enter a repl once the program exits
    pub interactive: bool,

    /// Disable colored output
    pub no_color: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            file_name: None,
            interactive: false,
            no_color: false,
        }
    }
}

impl Options {
    pub fn from_args() -> Option<Self> {
        let mut options = Options::default();

        // first argument is the path to the executable.
        // docs say we shouldn't rely on this behaviour - how tf are
        // we supposed to deal with arguments randomly appearing
        // in any sane way????
        for arg in env::args().skip(1) {
            match arg.as_ref() {
                "--help" | "-h" => {
                    println!("{}", info());
                    println!("{}", HELP);
                    return None;
                }
                "--version" | "-V" => {
                    println!("{}", info());
                    return None;
                }
                "--interactive" | "-i" => options.interactive = true,
                "--no-color" | "-nc" => options.no_color = true,
                x => options.file_name = Some(PathBuf::from(x)),
            }
        }
        Some(options)
    }
}
