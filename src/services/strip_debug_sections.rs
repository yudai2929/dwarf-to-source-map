use crate::models::wasm::Wasm;

pub fn strip_debug_sections(wasm: &Wasm) -> Wasm {
    let mut pos = 8;
    let mut stripped = wasm[..pos].to_vec();

    while pos < wasm.len() {
        let section_start = pos;
        let (section_id, pos_) = read_var_uint(&wasm, pos);
        let (section_size, section_body) = read_var_uint(&wasm, pos_);
        pos = section_body + section_size as usize;
        if section_id == 0 {
            let (name_len, name_pos) = read_var_uint(&wasm, section_body);
            let name_end = name_pos + name_len as usize;
            let name = &wasm[name_pos..name_end];
            if name == b"linking"
                || name == b"sourceMappingURL"
                || name.starts_with(b"reloc..debug_")
                || name.starts_with(b".debug_")
            {
                continue; // skip debug related sections
            }
        }
        stripped.extend_from_slice(&wasm[section_start..pos]);
    }

    stripped
}

fn read_var_uint(wasm: &Wasm, pos: usize) -> (u64, usize) {
    let mut n = 0;
    let mut shift = 0;
    let mut b = wasm[pos] as u64;
    let mut _pos = pos + 1;
    while b >= 128 {
        n |= (b - 128) << shift;
        b = wasm[_pos] as u64;
        _pos += 1;
        shift += 7;
    }
    (n + (b << shift), _pos)
}
