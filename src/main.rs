use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

use markdown::{
    self,
    mdast::{Heading, List, Node},
};

#[derive(Debug, PartialEq)]
struct ToCheckItem {
    text: String,
    link: String,
}

impl Display for ToCheckItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.text, self.link)
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("no file given!");
        exit(1);
    }

    let path = args.get(1).unwrap();

    let md = read_file(path).expect("Error reading file");

    let items = parse_markdown(&md);

    for item in items.iter() {
        println!("{}", item);
    }
    Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn parse_markdown(md: &str) -> Vec<ToCheckItem> {
    let root = markdown::to_mdast(md, &markdown::ParseOptions::default()).unwrap();

    let mut header_found = false;
    let mut items: Vec<ToCheckItem> = Vec::new();

    for c in root.children().iter() {
        for cc in c.iter() {
            match cc {
                markdown::mdast::Node::Heading(h) if get_header_title(h) == "Things to check" => {
                    header_found = true;
                }
                markdown::mdast::Node::List(l) if header_found => {
                    items = get_list_items(l);
                }
                _ => {}
            }
        }
    }

    items
}

fn get_list_items(l: &List) -> Vec<ToCheckItem> {
    l.children
        .iter()
        .flat_map(|n| match n {
            Node::ListItem(li) => Some(li),
            _ => None,
        })
        .flat_map(|li| {
            li.children.iter().filter_map(|p| match p {
                Node::Paragraph(pp) => Some(pp),
                _ => None,
            })
        })
        .flat_map(|pp| {
            pp.children.iter().filter_map(|ll| match ll {
                Node::Link(lll) => Some(lll),
                _ => None,
            })
        })
        .flat_map(|lll| {
            lll.children.iter().filter_map(|t| match t {
                Node::Text(tt) => Some(ToCheckItem {
                    text: tt.value.to_string(),
                    link: lll.url.to_string(),
                }),
                _ => None,
            })
        })
        .collect()
}

fn get_header_title(h: &Heading) -> String {
    h.children
        .iter()
        .filter_map(|c| {
            if let Node::Text(cc) = c {
                Some(cc.value.to_string())
            } else {
                None
            }
        })
        .next()
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let md = "## Things to check

> Move to a correct place after review.

- [Link text](http://link.com)
- [Link2 text](http://link2.com)";

        let expected = vec![
            ToCheckItem {
                text: "Link text".to_string(),
                link: "http://link.com".to_string(),
            },
            ToCheckItem {
                text: "Link2 text".to_string(),
                link: "http://link2.com".to_string(),
            },
        ];

        let result = parse_markdown(md);

        assert_eq!(result, expected);
    }
}
