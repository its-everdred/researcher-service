use pulldown_cmark::{Parser, Event, Tag};
use std::fs;
use std::path::Path;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EipChunk {
    pub eip_id: u32,
    pub title: String,
    pub authors: Vec<String>,
    pub section: String,
    pub content: String,
    pub source_path: String,
}

pub fn parse_eip_chunks(id: u32) -> Result<Vec<EipChunk>, String> {
    let file_path = format!("data/EIPs/EIPS/eip-{}.md", id);
    let contents = fs::read_to_string(Path::new(&file_path))
        .map_err(|e| format!("Failed to read EIP file: {}", e))?;

    let authors = extract_authors(&contents);
    let title = extract_title(&contents);

    let parser = Parser::new(&contents);
    let mut chunks = Vec::new();
    let mut current_section = String::new();
    let mut current_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading(..)) => {
                if !current_content.is_empty() {
                    chunks.push(EipChunk {
                        eip_id: id,
                        title: title.clone(),
                        authors: authors.clone(),
                        section: current_section.clone(),
                        content: current_content.clone(),
                        source_path: file_path.clone(),
                    });
                    current_content.clear();
                }
            }
            Event::Text(text) => current_content.push_str(&text),
            Event::End(Tag::Heading(..)) => {
                current_section = current_content.clone();
                current_content.clear();
            }
            _ => {}
        }
    }

    if !current_content.is_empty() {
        chunks.push(EipChunk {
            eip_id: id,
            title,
            authors,
            section: current_section,
            content: current_content,
            source_path: file_path,
        });
    }

    Ok(chunks)
}

fn extract_title(contents: &str) -> String {
    contents
        .lines()
        .find(|line| line.starts_with("title:"))
        .map(|line| line.replace("title:", "").trim().to_string())
        .unwrap_or_else(|| "Unknown Title".to_string())
}

fn extract_authors(contents: &str) -> Vec<String> {
    contents
        .lines()
        .find(|line| line.starts_with("author:"))
        .map(|line| {
            line.replace("author:", "")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        })
        .unwrap_or_else(|| vec!["Unknown Author".to_string()])
}
