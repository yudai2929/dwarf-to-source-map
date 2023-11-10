use models::dwarf::Dwarf;
use models::entry::Entry;
use regex::Regex;
use std::collections::HashMap;

use crate::{models, utils};

pub fn parse_dwarf_to_entries(dwarf: &Dwarf) -> Vec<Entry> {
    let debug_line_chunks = convert_to_debug_lien_chunks(dwarf);

    let maybe_debug_info_content = debug_line_chunks.get(0);

    let mut entries: Vec<Entry> = Vec::new();

    if maybe_debug_info_content.is_none() {
        return entries;
    }

    let debug_info_content = maybe_debug_info_content.unwrap();

    for i in (1..debug_line_chunks.len()).step_by(2) {
        let stmt_list = debug_line_chunks[i].clone();
        let comp_dir = get_comp_dir(&debug_info_content, &stmt_list);

        let line_chunk = debug_line_chunks[i + 1].clone();

        let include_directories = get_include_directories(&line_chunk, &comp_dir);

        let file_paths = get_file_paths(&line_chunk, &include_directories);

        for line in Regex::new(r"\n0x([0-9a-f]+)\s+(\d+)\s+(\d+)\s+(\d+)(.*?end_sequence)?")
            .unwrap()
            .captures_iter(&line_chunk)
        {
            let address = i64::from_str_radix(&line[1], 16).unwrap();
            let entry_line = line[2].parse().unwrap();
            let column = line[3].parse().unwrap();
            let file_path = file_paths[&line[4]].clone();
            let eos = line.get(5).is_some();

            if !eos {
                entries.push(create_entry(address, entry_line, column, file_path, eos));
                continue;
            }

            let decremented_address = address - 1;

            if entries
                .last()
                .map_or(true, |last_entry| last_entry.address != decremented_address)
            {
                entries.push(create_entry(
                    decremented_address,
                    entry_line,
                    column,
                    file_path,
                    true,
                ));
                continue;
            }

            update_last_entry(&mut entries);
        }
    }
    remove_dead_entries(&mut entries);
    entries
}

fn convert_to_debug_lien_chunks(dwarf: &Dwarf) -> Vec<String> {
    let regex = Regex::new(r"debug_line\[(0x[0-9a-f]*)\]").unwrap();
    let decoded_dwarf = String::from_utf8(dwarf.to_vec()).unwrap();
    let chunks = utils::regex::split_keep(&regex, &decoded_dwarf);
    chunks.iter().map(|s| s.to_string()).collect()
}

fn get_comp_dir(debug_info_content: &str, stmt_list: &str) -> String {
    let comp_dir_match = Regex::new(&format!(
        "{}{}{}{}",
        r#"DW_AT_stmt_list\s+\("#,
        regex::escape(stmt_list),
        r#"\)\s+"#,
        r#"DW_AT_comp_dir\s+\(\"([^\"]+)"#
    ))
    .unwrap()
    .captures(debug_info_content);

    if let Some(mat) = comp_dir_match {
        mat.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        "".to_string()
    }
}

fn get_include_directories(line_chunk: &str, comp_dir: &str) -> HashMap<String, String> {
    let mut include_directories: HashMap<String, String> = HashMap::new();
    include_directories.insert("0".to_string(), comp_dir.to_string());

    for dir in Regex::new(r#"include_directories\[\s*(\d+)\] = \"([^\"]*)"#)
        .unwrap()
        .captures_iter(line_chunk)
    {
        include_directories.insert(dir[1].to_string(), dir[2].to_string());
    }

    include_directories
}

fn get_file_paths(
    line_chunk: &str,
    include_directories: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut file_paths: HashMap<String, String> = HashMap::new();
    for file in Regex::new(r#"file_names\[\s*(\d+)\]:\s+name: \"([^\"]*)\"\s+dir_index: (\d+)"#)
        .unwrap()
        .captures_iter(line_chunk)
    {
        let dir = include_directories[&file[3]].clone();
        let file_path = if file[2].chars().next().unwrap() != '/' {
            format!("{}/{}", dir, file[2].to_string())
        } else {
            file[2].to_string()
        };
        file_paths.insert(file[1].to_string(), file_path);
    }

    file_paths
}

fn create_entry(address: i64, line: i32, column: i32, file_path: String, eos: bool) -> Entry {
    Entry {
        address,
        line,
        column,
        file_path,
        eos,
    }
}

fn update_last_entry(entries: &mut Vec<Entry>) -> () {
    if let Some(last_entry) = entries.last_mut() {
        last_entry.eos = true;
    }
}

fn remove_dead_entries(entries: &mut Vec<Entry>) {
    let mut block_start = 0;
    let mut cur_entry = 0;
    while cur_entry < entries.len() {
        if !entries[cur_entry].eos {
            cur_entry += 1;
            continue;
        }
        let fn_start = entries[block_start].address;
        let fn_size_length =
            f64::log((entries[cur_entry].address - fn_start + 1) as f64, 128.0).floor() as i64 + 1;
        let min_live_offset = 1 + fn_size_length;
        if fn_start < min_live_offset {
            entries.drain(block_start..=cur_entry);
            cur_entry = block_start;
            continue;
        }
        cur_entry += 1;
        block_start = cur_entry;
    }
}
