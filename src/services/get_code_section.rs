use crate::models::wasm;

pub fn get_code_section_offset(wasm: &wasm::Wasm) -> Result<i64, String> {
    let mut pos = 8;

    while pos < wasm.len() {
        let (section_id, _pos) = read_var_uint32(wasm, pos);
        let (section_size, _pos) = read_var_uint32(wasm, _pos);
        if section_id == 10 {
            return Ok(_pos as i64);
        }
        pos = _pos + section_size;
    }

    return Err("Code section not found".to_string());
}

fn read_var_uint32(wasm: &wasm::Wasm, pos: usize) -> (usize, usize) {
    let mut n = 0;
    let mut shift = 0;
    let mut b = wasm[pos..pos + 1].to_vec()[0];
    let mut pos = pos + 1;
    while b >= 128 {
        n = n | ((b - 128) as usize) << shift;
        b = wasm[pos..pos + 1].to_vec()[0];
        shift += 7;
        pos += 1;
    }

    return (n | (b as usize) << shift, pos);
}
