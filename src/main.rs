mod interpreter;
// mod ast;

use interpreter::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(default_value = "sim")]
    mode: String,
    #[structopt(parse(from_os_str))]
    #[structopt(required_if("mode", "com"))]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Arguments::from_args();
    println!("{:?}", args);
    let program = Program {
    };


    if args.mode == "sim" {
        simulate(program);
    }
    else if args.mode == "com" {
        compile(program);
    }
    else {
        panic!("Unknown mode: {}", args.mode);
    }

}
