use std::fs;
use std::path::Path;

pub fn parse_eip(id: u32) -> String {
    let file_path = format!("data/EIPs/EIPS/eip-{}.md", id);

    // Try to read the file
    match fs::read_to_string(Path::new(&file_path)) {
        Ok(contents) => contents,
        Err(_) => format!("EIP {} not found", id),
    }
}
