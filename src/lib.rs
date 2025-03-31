use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use anyhow::{Result, anyhow};
use regex::Regex;

// Structs for notes and notebooks
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub name: String,
    pub contents: String,
    pub links: Vec<String>,        // @link_to_note
    pub tags: HashSet<String>,     // #tags
    pub backlinks: Vec<String>,    // Computed later
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    pub notes: HashMap<String, Note>, // Keyed by note_name
}

// Parser
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "note.pest"]
struct NoteParser;

pub fn parse_note(text: &str) -> Result<Note, anyhow::Error> {
    let pairs = NoteParser::parse(Rule::note, text)
        .map_err(|e| anyhow!("1: Failed to parse note: {}", e))?;

    let note_pair = pairs.into_iter().next()
        .ok_or_else(|| anyhow!("2: No valid note structure found in text"))?;

    let mut name = String::new();
    let mut contents = String::new();

    for pair in note_pair.into_inner() {
        match pair.as_rule() {
            Rule::name => name = pair.as_str().to_string(),
            Rule::contents => contents = pair.as_str().to_string(),
            _ => {}
        }
    }

    if name.is_empty() {
        return Err(anyhow!("3: Note must have a non-empty name"));
    }

    // Extract links and tags from contents
    let link_re = match Regex::new(r"@[a-zA-Z]+") {
        Ok(re) => re,
        Err(e) => return Err(anyhow!("4: Failed to compile regex for links: {}", e)),
    };
    let tag_re = match Regex::new(r"@[a-zA-Z]+") {
        Ok(re) => re,
        Err(e) => return Err(anyhow!("4: Failed to compile regex for links: {}", e)),
    };

    let links: Vec<String> = link_re.find_iter(&contents)
        .map(|m| m.as_str()[1..].to_string())  // Strip @
        .collect();

    let tags: HashSet<String> = tag_re.find_iter(&contents)
        .map(|m| m.as_str()[1..].to_string())  // Strip #
        .collect();

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
    use std::fs;
    use std::path::PathBuf;

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
