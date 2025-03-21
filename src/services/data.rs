use std::fs;
use std::path::Path;

pub fn eip_data(id: u32) -> String {
    let file_path = format!("data/EIPs/EIPS/eip-{}.md", id);

    match fs::read_to_string(Path::new(&file_path)) {
        Ok(contents) => contents,
        Err(_) => format!("EIP {} not found", id),
    }
}
