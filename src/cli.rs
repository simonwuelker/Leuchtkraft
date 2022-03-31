use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Leuchtkraft", author)]
pub struct Options {
    /// Path to a executable Leuchtkraft script
    #[structopt(parse(from_os_str))]
    pub file_name: Option<PathBuf>,

    /// Enter a repl once the program exits
    #[structopt(short, long)]
    pub interactive: bool,

    /// Disable colored output
    #[structopt(short, long)]
    pub no_color: bool,
}
