mod models;
mod services;
mod utils;

extern crate clap;

use clap::Parser;
use models::dwarf::Dwarf;
use models::wasm::Wasm;
use services::build_source_map::build_source_map;
use services::get_code_section::get_code_section_offset;
use services::parse_dwarf_to_entries::parse_dwarf_to_entries;

use crate::services::append_source_mapping::append_source_mapping;
use crate::services::strip_debug_sections::strip_debug_sections;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandLineArgs {
    /// Input WASM file path
    #[arg(long = "input-wasm-file-path")]
    input_wasm_file_path: String,

    /// Input DWARF file path
    #[arg(long = "input-dwarf-file-path")]
    input_dwarf_file_path: String,

    /// Output Source Map file path
    #[arg(long = "output-source-map-file-path")]
    output_source_map_file_path: String,

    /// Output WASM file path
    #[arg(long = "output-wasm-file-path")]
    output_wasm_file_path: Option<String>,

    /// Source Map URL
    #[arg(long = "source-map-url")]
    source_map_url: String,

    /// Removes debug info and linking sections
    #[arg(long = "stripped", default_value = "false")]
    stripped: bool,

    /// Read and embed source files from file system into source map
    #[arg(long = "is-embed-sources", default_value = "false")]
    is_embed_sources: bool,

    /// Base path for source files, which will be relative to this
    #[arg(long = "base-path")]
    base_path: Option<String>,

    /// Replace source debug filename prefix for source map
    #[arg(long = "source_prefix")]
    source_prefix: Vec<String>,
}

fn main() {
    println!("Start generating source map...");
    let args = CommandLineArgs::parse();

    let mut wasm: Wasm = std::fs::read(&args.input_wasm_file_path).unwrap();
    let dwarf: Dwarf = std::fs::read(&args.input_dwarf_file_path).unwrap();

    let entries = parse_dwarf_to_entries(&dwarf);
    let code_section_offset = get_code_section_offset(&wasm).unwrap();

    let source_map = build_source_map(entries, code_section_offset, args.is_embed_sources);

    std::fs::write(
        &args.output_source_map_file_path,
        serde_json::to_string(&source_map).unwrap(),
    )
    .unwrap();

    if args.stripped {
        wasm = strip_debug_sections(&wasm);
    }

    if args.source_map_url != "" {
        wasm = append_source_mapping(&wasm, &args.source_map_url);
    }

    if args.output_wasm_file_path.is_some() {
        std::fs::write(&args.output_wasm_file_path.unwrap(), wasm).unwrap();
    }

    println!("Finish generating source map...");
}
