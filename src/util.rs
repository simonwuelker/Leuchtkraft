use pest::error::{Error, LineColLocation, ErrorVariant};

pub fn print_parse_error<R: std::fmt::Debug>(err: Error<R>, file: &str, filename: &str) {
    match err.variant {
        ErrorVariant::ParsingError{positives, negatives} => {
            println!("Parse Error:");

            if positives.len() == 1 {
                println!("Expected {:?}.", positives[0]);
            } else {
                println!("Expected any of {:?}", positives)
            }
        },
        ErrorVariant::CustomError{message} => {
            println!("Custom Error:");
            println!("{}", message);
        }
    }

    match err.line_col {
        LineColLocation::Pos(pos) => {
            let faulty_line = file.lines().nth(pos.0 - 1).unwrap();
            println!("--> {}:{}:{}", filename, pos.0, pos.1);
            println!("{}", faulty_line);
            println!("{}^", " ".repeat(pos.1 - 1));
            println!("{}|", " ".repeat(pos.1 - 1));
        },
        LineColLocation::Span(start, end) => {
            unimplemented!();
        },
    }
}
