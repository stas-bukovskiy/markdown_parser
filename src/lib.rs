extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"]
struct MarkdownParser;

pub fn parse_markdown(markdown_text: &str) -> String {
    let pairs = MarkdownParser::parse(Rule::markdown, markdown_text).unwrap_or_else(|e| panic!("{}", e));

    let mut html = String::new();

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::h1 => {
                    html.push_str(&format!("<h1>{}</h1>", inner_pair.as_str().replace("# ", "")));
                }
                Rule::h2 => {
                    html.push_str(&format!("<h2>{}</h2>",  inner_pair.as_str().replace("## ", "")));
                }
                Rule::paragraph => {
                    html.push_str(&format!("<p>{}</p>",  inner_pair.as_str()));
                }
                _ => {}
            };
        }
    }

    html
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let markdown_text = "# Heading 1\n## Heading 2\n\nThis is a paragraph.";
    //     let html = parse_markdown(&markdown_text);
    //     println!("{}", html);
    // }


    #[test]
    fn test_valid_header() {
        let markdown_text = "# Header 1";
        let html = parse_markdown(markdown_text);
        assert_eq!(html, "<h1>Header 1</h1>");
    }

    #[test]
    fn test_valid_paragraph() {
        let markdown_text = "This is a paragraph";
        let html = parse_markdown(markdown_text);
        assert_eq!(html, "<p>This is a paragraph</p>");
    }

    #[test]
    fn test_invalid_header() {
        let markdown_text = "#Invalid Header";
        assert_eq!(markdown_text, "#Invalid Header");
    }
}
