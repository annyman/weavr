use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use anyhow::{Result, anyhow};
use std::fs;

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
        .map_err(|e| anyhow!("1: Parse error: {}", e))?;

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
        return Err(anyhow!("2: Name cannot be empty"));
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

pub fn split_and_parse_notes(text: &str) -> Result<Notebook, anyhow::Error> {
    let mut notebook = Notebook {
        notes: HashMap::new(),
    };

    let text = text.trim();
    println!("Text: {}", text);

    // Split at "}" and rebuild each note
    let mut current_note = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        current_note.push_str(trimmed);
        if trimmed.ends_with("}") {
            let note = parse_note(trimmed)?;
            notebook.notes.insert(note.name.clone(), note);
            current_note.clear();
        } else {
            current_note.push_str(" ");
        }
    }

    if !current_note.is_empty() {
        let note = parse_note(&current_note)?;
        notebook.notes.insert(note.name.clone(), note);
    }

    // Compute backlinks
    let mut backlinks: HashMap<String, Vec<String>> = HashMap::new();
    for note in notebook.notes.values() {
        for link in &note.links {
            backlinks.entry(link.clone())
                .or_insert(Vec::new())
                .push(note.name.clone());
        }
    }
    for (name, note) in notebook.notes.iter_mut() {
        if let Some(links) = backlinks.get(name) {
            note.backlinks = links.clone();
        }
    }

    Ok(notebook)
}

pub fn serialize_notebook_to_json(notebook: &Notebook) -> Result<String, anyhow::Error> {
    serde_json::to_string_pretty(notebook)
        .map_err(|e| anyhow!("3: JSON serialization error: {}", e))
}

pub fn process_note_file(file_path: &str) -> Result<String, anyhow::Error> {
    let text = fs::read_to_string(file_path)
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path, e))?;
    
    let notebook = split_and_parse_notes(&text)?;
    
    serde_json::to_string_pretty(&notebook)
        .map_err(|e| anyhow!("JSON serialization error: {}", e))
}

pub fn write_json_to_file(json: &str, output_path: &str) -> Result<(), anyhow::Error> {
    fs::write(output_path, json)
        .map_err(|e| anyhow!("Failed to write to {}: {}", output_path, e))
}
