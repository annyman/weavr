use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub name: String,
    pub contents: String,
    pub links: Vec<String>,
    pub tags: HashSet<String>,
    pub backlinks: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    pub notes: HashMap<String, Note>,
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "note.pest"]
pub struct NoteParser;

pub fn parse_note(text: &str) -> Result<Note, anyhow::Error> {
    let pairs = NoteParser::parse(Rule::note, text)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    let mut name = String::new();
    let mut contents = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::note => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::name => name = inner_pair.as_str().to_string(),
                        Rule::contents => contents = inner_pair.as_str().to_string(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if name.is_empty() {
        return Err(anyhow!("Name cannot be empty"));
    }

    let mut links = Vec::new();
    let mut tags = HashSet::new();

    // Split contents into words and check Python-style
    for word in contents.split_whitespace() {
        if word.starts_with("@") {
            links.push(word[1..].to_string());
        } else if word.starts_with("#") {
            tags.insert(word[1..].to_string());
        }
    }

    Ok(Note {
        name,
        contents,
        links,
        tags,
        backlinks: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note() {
        let text = "bruh { contents go here and i can link to @note2 and add tag of #some_tag }";
        let note = match parse_note(text) {
            Ok(note) => note,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(note.name, "bruh");
        assert_eq!(note.contents, "contents go here and i can link to @note2 and add tag of #some_tag");
        assert_eq!(note.links, vec!["note2"]);
        assert_eq!(note.tags, HashSet::from(["some_tag".to_string()]));
    }
}