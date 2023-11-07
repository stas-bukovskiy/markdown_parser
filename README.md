# Markdown Parser
This is a simple Markdown to HTML parser implemented in Rust using the pest parser generator library. It can convert Markdown text into HTML.
## Table of Contents
1. [Installation](#installation)
2. [Usage](#usage)
3. [Example](#example)
4. [Fourth Example](#fourth-examplehttpwwwfourthexamplecom)

## Installation
To use this Markdown to HTML parser in your Rust project, add it as a dependency in your Cargo.toml file:
```toml
[dependencies]
markdown_to_html_parser = "0.1.0"
```

## Usage
First, import the library in your Rust code:
```rust
use markdown_to_html_parser::parse_markdown;
```
Then, you can convert Markdown text to HTML using the parse_markdown function:
```rust
xlet markdown_text = "Your Markdown text here";
let html = parse_markdown(markdown_text);
println!("{}", html);
```

## Example
Here's an example of how to use the Markdown to HTML parser in a Rust program:
```rust
use markdown_to_html_parser::parse_markdown;

fn main() {
    let markdown_text = "# Heading 1\nSome *italic* and **bold** text.\n## Heading 2\nMore text.";
    let html = parse_markdown(markdown_text);
    println!("{}", html);
}
```
Running this program will produce the following HTML output:
```html
<h1>Heading 1</h1><p>Some <em>italic</em> and <strong>bold</strong> text.</p><h2>Heading 2</h2><p>More text.</p>
```

## Grammar

This parser uses a simple grammar to recognize Markdown elements. The grammar is defined in the `markdown.pest` file, and it includes rules for headers and paragraphs. Here's a brief overview of the grammar: 
 * `markdown` consists of a sequence of headers (h1 and h2) and paragraphs separated by newline characters.
 * `h1` matches lines starting with `#` followed by one or more alphanumeric or space characters.
 * `h2` matches lines starting with `##` followed by one or more alphanumeric or space characters.
 * `paragraph` matches any line that doesn't match the above rules.

## Contributing
Contributions are welcome! If you want to improve the parser, fix bugs, or add new features, please open an issue or submit a pull request on the [GitHub repository](https://github.com/stas-bukovskiy/markdown_parser).

## License
[TODO] This Markdown to HTML parser is open-source and licensed under the MIT License. See the LICENSE file for more details.
