use std::fmt::Display;

use markdown::{
    self,
    mdast::{Heading, List, Node},
};

#[derive(Debug)]
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
    let md = "## Things to check

> Move to a correct place after review.

- [Visualizing memory layout of Rust's data types](https://www.youtube.com/watch?v=7_o-YRxf_cc&t=0s)
- [Oma sivu](https://juhosalli.fi)";

    let root = markdown::to_mdast(md, &markdown::ParseOptions::default())?;
    // println!("{:?}", root);

    // println!("---");
    // println!("child count {}", root.children().iter().len());

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

    for item in items.iter() {
        println!("{}", item);
    }

    Ok(())
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

// fn get_list_items(l: &List) -> Vec<ToCheckItem> {
//     let mut items: Vec<ToCheckItem> = Vec::new();

//     for n in l.children.iter() {
//         match n {
//             Node::ListItem(li) => {
//                 // ListItem -> Paragraph -> Link (url) -> Text (value)
//                 for p in li.children.iter() {
//                     match p {
//                         Node::Paragraph(pp) => {
//                             for ll in pp.children.iter() {
//                                 match ll {
//                                     Node::Link(lll) => {
//                                         // println!("URL: {}", lll.url);
//                                         for t in lll.children.iter() {
//                                             match t {
//                                                 Node::Text(tt) => {
//                                                     // println!("value: {}", tt.value);
//                                                     let ttc: ToCheckItem = ToCheckItem {
//                                                         text: tt.value.to_string(),
//                                                         link: lll.url.to_string(),
//                                                     };
//                                                     items.push(ttc);
//                                                 }
//                                                 _ => {}
//                                             }
//                                         }
//                                     }
//                                     _ => {}
//                                 }
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }

//     items
// }

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
    // for c in h.children.iter() {
    //     match c {
    //         Node::Text(cc) => return cc.value.to_string(),
    //         _ => return "".to_string(),
    //     }
    // }

    // "".to_string()
}
