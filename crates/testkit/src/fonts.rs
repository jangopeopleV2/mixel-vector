//! Font files for tests, made from the bundled fonts when a test runs: no font file is committed.

/// The bundled Noto Sans Regular with its family renamed in every name record.
/// The family may have any length; its PostScript name stays `NotoSans-Regular`.
pub fn renamed(family: &str) -> Vec<u8> {
    let data = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");
    vectorcraft_text::test_fonts::renamed_font(data, &[("Noto Sans", family)]).unwrap()
}

/// A resource fork holding `fonts` as `sfnt` resources and one other resource (a `vers`), as a font
/// utility writes a suitcase font's fork, and as vectorcraft-text's suitcase tests build one. macOS
/// keeps a file's resource fork at `<file>/..namedfork/rsrc`.
pub fn suitcase_fork(fonts: &[Vec<u8>]) -> Vec<u8> {
    let mut data = Vec::new();
    let mut offsets = Vec::new();
    for body in fonts.iter().map(Vec::as_slice).chain([&b"1.0"[..]]) {
        offsets.push(data.len() as u32);
        data.extend_from_slice(&(body.len() as u32).to_be_bytes());
        data.extend_from_slice(body);
    }
    let n = fonts.len();
    // The type list: `vers` (1 resource) then `sfnt`, each entry pointing at its references.
    let list = 2 + 2 * 8;
    let mut types = Vec::new();
    types.extend_from_slice(&1u16.to_be_bytes());
    types.extend_from_slice(b"vers");
    types.extend_from_slice(&0u16.to_be_bytes());
    types.extend_from_slice(&(list as u16).to_be_bytes());
    types.extend_from_slice(b"sfnt");
    types.extend_from_slice(&((n - 1) as u16).to_be_bytes());
    types.extend_from_slice(&((list + 12) as u16).to_be_bytes());
    let reference = |id: i16, offset: u32| {
        let mut r = Vec::new();
        r.extend_from_slice(&id.to_be_bytes());
        r.extend_from_slice(&0xFFFFu16.to_be_bytes());
        r.extend_from_slice(&offset.to_be_bytes());
        r.extend_from_slice(&[0; 4]);
        r
    };
    types.extend(reference(1, offsets[n]));
    for (i, o) in offsets[..n].iter().enumerate() {
        types.extend(reference(5000 + i as i16, *o));
    }
    // The map: a copy of the header, a handle, attributes, then the offsets of the lists.
    let mut map = vec![0; 24];
    map.extend_from_slice(&28u16.to_be_bytes());
    map.extend_from_slice(&((28 + types.len()) as u16).to_be_bytes());
    map.extend(types);
    let mut fork = Vec::new();
    for v in [256u32, 256 + data.len() as u32, data.len() as u32, map.len() as u32] {
        fork.extend_from_slice(&v.to_be_bytes());
    }
    fork.resize(256, 0);
    fork.extend(data);
    fork.extend(map);
    fork
}
