// use pest::error::{Error, LineColLocation, ErrorVariant};
use crate::debug::annotation::DisplaySnippet;
use annotate_snippets::display_list::DisplayList;

pub fn print_snippet<T: DisplaySnippet>(snippet: T) {
    let dl = DisplayList::from(snippet.as_snippet());
    println!("{}", dl);
}
