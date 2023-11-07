extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate html_escape;

use pest::Parser;
use html_escape::encode_text;

#[derive(Parser)]
#[grammar = "markdown.pest"]
struct MarkdownParser;

fn main() {
    let input = "This is *italic* and **bold** text.";

    let pairs = MarkdownParser::parse(Rule::document, input).expect("Failed to parse input.");

    let mut output = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::text => {
                output += &encode_text(pair.as_str());
            }
            Rule::italic => {
                output += &format!("<em>{}</em>", pair.as_str());
            }
            Rule::bold => {
                output += &format!("<strong>{}</strong>", pair.as_str());
            }
            _ => {}
        }
    }

    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
