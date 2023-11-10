use crate::models::wasm::Wasm;

pub fn append_source_mapping(wasm: &Wasm, url: &str) -> Wasm {
    let section_name = b"sourceMappingURL";

    let mut section_content = Vec::new();
    section_content.extend_from_slice(&encode_uint_var(section_name.len() as u64));
    section_content.extend_from_slice(section_name);
    section_content.extend_from_slice(&encode_uint_var(url.len() as u64));
    section_content.extend_from_slice(url.as_bytes());

    let mut result = wasm.to_vec();
    result.extend_from_slice(&encode_uint_var(0));
    result.extend_from_slice(&encode_uint_var(section_content.len() as u64));
    result.extend_from_slice(&section_content);
    result
}

fn encode_uint_var(mut n: u64) -> Vec<u8> {
    let mut result = Vec::new();
    while n > 127 {
        result.push(128 | (n & 127) as u8);
        n = n >> 7;
    }
    result.push(n as u8);
    result
}
