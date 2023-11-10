use crate::models::{entry::Entry, source_map::SourceMap};
use crate::utils::normalize_path::normalize_path;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn build_source_map(
    entries: Vec<Entry>,
    code_section_offset: i64,
    is_embed_sources: bool,
) -> SourceMap {
    let mut sources: Vec<String> = Vec::new();
    let mut sources_content: Option<Vec<Option<String>>> = if is_embed_sources {
        Some(Vec::new())
    } else {
        None
    };
    let mut mappings: Vec<String> = Vec::new();
    let mut sources_map: HashMap<String, i32> = HashMap::new();
    let mut last_address = 0;
    let mut last_source_id = 0;
    let mut last_line = 1;
    let mut last_column = 1;

    for entry in entries {
        let line = entry.line;
        if line == 0 {
            continue;
        }

        let mut column = entry.column;
        if column == 0 {
            column = 1;
        }

        let address = entry.address + code_section_offset;
        let file_name = normalize_path(&entry.file_path);

        let source_name = file_name.clone();

        let source_id: i32 = if !sources_map.contains_key(&source_name) {
            let id: i32 = sources.len() as i32;
            sources_map.insert(source_name.clone(), id);
            sources.push(source_name.clone());
            if is_embed_sources {
                let source_content = File::open(&file_name)
                    .and_then(|mut file| {
                        let mut content = String::new();
                        file.read_to_string(&mut content)?;
                        Ok(content)
                    })
                    .ok();
                sources_content.as_mut().unwrap().push(source_content);
            }
            id
        } else {
            *sources_map.get(&source_name).unwrap()
        };

        let address_delta = address - last_address;
        let source_id_delta = source_id - last_source_id;
        let line_delta = line - last_line;
        let column_delta = column - last_column;

        mappings.push(
            encode_vlq(address_delta as i32)
                + &encode_vlq(source_id_delta)
                + &encode_vlq(line_delta)
                + &encode_vlq(column_delta),
        );
        last_address = address;
        last_source_id = source_id;
        last_line = line;
        last_column = column;
    }

    SourceMap {
        version: 3,
        names: Vec::new(),
        sources,
        sources_content,
        mappings: mappings.join(","),
    }
}

fn encode_vlq(n: i32) -> String {
    const VLQ_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut x = if n >= 0 { n << 1 } else { (-n << 1) + 1 };
    let mut result = String::new();
    while x > 31 {
        result.push(VLQ_CHARS.chars().nth(32 + (x & 31) as usize).unwrap());
        x = x >> 5;
    }
    result.push(VLQ_CHARS.chars().nth(x as usize).unwrap());
    result
}
