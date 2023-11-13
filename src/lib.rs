extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair};
use pest::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"]
struct MarkdownParser;

pub fn parse_markdown(markdown_text: &str) -> String {
    let pairs = MarkdownParser::parse(Rule::markdown, markdown_text).unwrap_or_else(|e| panic!("{}", e));

    let mut html = String::new();

    for markdown in pairs.into_iter() {
        // go inside markdown
        for markdown_inner in markdown.into_inner() {
            match markdown_inner.as_rule() {
                Rule::elements => {
                    html.push_str(&parser_elements( &markdown_inner.clone()));
                }
                Rule::element_inner => {
                    html.push_str(&parse_element_inner( &markdown_inner.clone()));
                }
                Rule::text => {
                    html.push_str(&parse_text( &markdown_inner.clone()));
                }
                Rule::new_line => {
                   html.push_str("<br>\n");
                }
                _ => {

                }
            }
        }
    }

    html
}

fn parser_elements(elements: &Pair<Rule>) -> String {
    let mut html = String::new();

    for element in elements.clone().into_inner() {
        match element.as_rule(){
            Rule::h1 => {
                html.push_str(&parse_h(&element,  "h1"));
            }
            Rule::h2 => {
                html.push_str(&parse_h(&element,  "h2"));
            }
            Rule::italic_text => {
                html.push_str(&parse_element(&element,  "i"));
            }
            Rule::bold_text => {
                html.push_str(&parse_element(&element,  "b"));
            }
            Rule::code_text => {
                html.push_str(&parse_element(&element,  "code"));
            }
            _ => {}
        }
    }

    html
}

fn parse_element(element: &Pair<Rule>, tag_name: &str) -> String {
    let mut inner = String::new();

    for header_inner in element.clone().into_inner() {
        match header_inner.as_rule() {
            Rule::element_inner => {
                inner.push_str(&parse_element_inner(&header_inner));
            }
            _ => {}
        }
    }

    format!("<{}>{}</{}>", tag_name, inner, tag_name)
}


fn parse_h(header: &Pair<Rule>, tag_name: &str) -> String{
    let mut inner = String::new();

    for header_inner in header.clone().into_inner() {
        match header_inner.as_rule() {
            Rule::elements => {
                inner.push_str(&parser_elements(&header_inner));
            }
            Rule::element_inner => {
                inner.push_str(&parse_element_inner(&header_inner));
            }
            Rule::text => {
                inner.push_str(&parse_text(&header_inner));
            }
            _ => {}
        }
    }
    format!("<{}>{}</{}>\n", tag_name, inner, tag_name)
}

fn parse_text(text: &Pair<Rule>) -> String {
    text.as_str().to_string()
}

fn parse_element_inner(element_inner: &Pair<Rule>) -> String {
    element_inner.as_str().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let markdown_text = "# Heading 1\n";
        // let markdown_text = "# Heading 1\n## Heading 2\n\nThis is a paragraph.";
        let html = parse_markdown(&markdown_text);
        println!("{}", html);
    }


    #[test]
    fn test_valid_simple_header() {
        let markdown_text = "# Header 1\n";
        let html = parse_markdown(markdown_text);
        assert_eq!(html, "<h1>Header 1</h1>\n");
    }

    #[test]
    fn test_valid_header_with_italic_code() {
        let markdown_text = "# Header 1 with _italic_ code\n";
        let html = parse_markdown(markdown_text);
        assert_eq!(html, "<h1>Header 1 with <i>italic</i> code</h1>\n");
    }

    #[test]
    fn test_valid_invalid_header() {
        let markdown_text = "# Header 1";
        let html = parse_markdown(markdown_text);
        assert_eq!(html, "# Header 1");
    }

    #[test]
    fn test_simple_markdown() {
        let markdown_text = "# Header 1\n## Header 2\n\nThis is a paragraph with _italic_ and **bold** text.";
        let html = parse_markdown(markdown_text);
        assert_eq!(html, "<h1>Header 1</h1>\n<h2>Header 2</h2>\n<br>\nThis is a paragraph with <i>italic</i> and <b>bold</b> text.");
    }

}
