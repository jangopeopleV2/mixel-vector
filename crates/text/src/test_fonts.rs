//! Fonts built in code for tests, so no font file is committed: a small variable font (#296).
//! Compiled for this crate's tests and, with the `test-fonts` feature, for other crates' tests.

use skrifa::instance::{LocationRef, Size};
use skrifa::raw::TableProvider;
use skrifa::raw::tables::glyf::Glyph;
use skrifa::{GlyphId, MetadataProvider};
use write_fonts::FontBuilder;
use write_fonts::tables::fvar::{AxisInstanceArrays, Fvar, InstanceRecord, VariationAxisRecord};
use write_fonts::tables::gvar::{GlyphDelta, GlyphDeltas, GlyphVariations, Gvar, Tent};
use write_fonts::types::{F2Dot14, Fixed, NameId, Tag};

/// Replace strings in a fixture font's `name` records and rebuild its SFNT tables.
/// Names may grow, shrink or contain Unicode: no equal-length byte replacement is
/// needed. Table offsets, lengths and checksums are written by `FontBuilder`.
/// Unicode and ASCII Mac Roman records are rewritten; other encodings stay intact.
pub fn renamed_font(data: &[u8], replacements: &[(&str, &str)]) -> Option<Vec<u8>> {
    let font = skrifa::FontRef::new(data).ok()?;
    let original = font.table_data(skrifa::Tag::new(b"name"))?;
    let original = original.as_bytes();
    let u16_at = |at: usize| -> Option<u16> { Some(u16::from_be_bytes(original.get(at..at + 2)?.try_into().ok()?)) };
    let (format, count, storage) = (u16_at(0)?, usize::from(u16_at(2)?), usize::from(u16_at(4)?));
    if format > 1 {
        return None;
    }
    let records_end = 6 + 12 * count;
    let languages = if format == 1 { usize::from(u16_at(records_end)?) } else { 0 };
    if records_end + (if format == 1 { 2 + 4 * languages } else { 0 }) > storage {
        return None;
    }
    let mut name = original.get(..storage)?.to_vec();
    let mut strings = Vec::new();
    for record in 0..count + languages {
        let (unicode, ascii, length_at, offset_at) = if record < count {
            let at = 6 + 12 * record;
            let platform = u16_at(at)?;
            (platform == 0 || platform == 3, platform == 1 && u16_at(at + 2)? == 0, at + 8, at + 10)
        } else {
            let at = records_end + 2 + 4 * (record - count);
            (true, false, at, at + 2)
        };
        let start = storage + usize::from(u16_at(offset_at)?);
        let bytes = original.get(start..start + usize::from(u16_at(length_at)?))?;
        let text = if unicode {
            if !bytes.len().is_multiple_of(2) {
                return None;
            }
            let units: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|v| u16::from_be_bytes(*v)).collect();
            Some(String::from_utf16(&units).ok()?)
        } else if ascii && bytes.is_ascii() {
            Some(String::from_utf8(bytes.to_vec()).ok()?)
        } else {
            None
        };
        let bytes = if let Some(mut text) = text {
            for &(from, to) in replacements {
                text = text.replace(from, to);
            }
            if unicode { text.encode_utf16().flat_map(u16::to_be_bytes).collect() } else { text.into_bytes() }
        } else {
            bytes.to_vec()
        };
        // Format 0/1 name records use 16-bit lengths and storage offsets.
        let length = u16::try_from(bytes.len()).ok()?;
        let offset = u16::try_from(strings.len()).ok()?;
        name.get_mut(length_at..length_at + 2)?.copy_from_slice(&length.to_be_bytes());
        name.get_mut(offset_at..offset_at + 2)?.copy_from_slice(&offset.to_be_bytes());
        strings.extend(bytes);
    }
    name.extend(strings);
    let mut builder = FontBuilder::new();
    builder.add_raw(Tag::new(b"name"), name);
    for record in font.table_directory.table_records() {
        let tag = Tag::new(&record.tag().to_be_bytes());
        if !builder.contains(tag) {
            builder.add_raw(tag, font.table_data(record.tag())?.as_bytes());
        }
    }
    Some(builder.build())
}

/// The family of [`variable_font`].
pub const VARIABLE_FAMILY: &str = "Varitest Sans";

/// The characters [`variable_font`] widens towards its heaviest weight (simple glyphs in Noto Sans; the composite ones, like `H`, would need their components varied).
pub const VARIABLE_CHARS: &str = "lIn";

/// How much wider [`VARIABLE_CHARS`] are at the heaviest weight (outlines and advances).
pub const VARIABLE_WIDEN: f32 = 0.25;

/// The bundled Noto Sans Regular as a variable font of family [`VARIABLE_FAMILY`]: one weight
/// axis (400–700, default 400, the face itself being Regular) whose maximum makes the glyphs of
/// [`VARIABLE_CHARS`] [`VARIABLE_WIDEN`] wider (outlines and advances, through `gvar`), with the
/// named instances Normal (the default, the face Regular), SemiBold (550) and Bold (700), plus an instance without
/// a name and one named like another (`BOLD`), which style lists leave out. `None` when it can't
/// be built.
pub fn variable_font() -> Option<Vec<u8>> {
    const BASE: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");
    let font = skrifa::FontRef::new(BASE).ok()?;
    let (glyf, loca) = (font.glyf().ok()?, font.loca(None).ok()?);
    let metrics = font.glyph_metrics(Size::unscaled(), LocationRef::default());
    let charmap = font.charmap();
    let widened: Vec<GlyphId> = VARIABLE_CHARS.chars().filter_map(|c| charmap.map(c)).collect();
    let mut variations = Vec::new();
    for g in 0..u32::from(font.maxp().ok()?.num_glyphs()) {
        let gid = write_fonts::types::GlyphId::new(g);
        let id = GlyphId::new(g);
        let deltas = if widened.contains(&id) {
            let Ok(Some(Glyph::Simple(s))) = loca.get_glyf(id, &glyf) else { return None };
            let wider = |x: f32| (x * VARIABLE_WIDEN).round() as i16;
            let mut d: Vec<GlyphDelta> = s.points().map(|p| GlyphDelta::required(wider(f32::from(p.x)), 0)).collect();
            // The phantom points: left side bearing, advance, top, bottom.
            let advance = wider(metrics.advance_width(id)?);
            d.extend([(0, 0), (advance, 0), (0, 0), (0, 0)].map(|(x, y)| GlyphDelta::required(x, y)));
            vec![GlyphDeltas::new(vec![Tent::new(F2Dot14::from_f32(1.0), None)], d)]
        } else {
            vec![]
        };
        variations.push(GlyphVariations::new(gid, deltas));
    }
    let gvar = Gvar::new(variations, 1).ok()?;
    let axis = VariationAxisRecord {
        axis_tag: Tag::new(b"wght"),
        min_value: Fixed::from_i32(400),
        default_value: Fixed::from_i32(400),
        max_value: Fixed::from_i32(700),
        flags: 0,
        axis_name_id: NameId::new(256),
    };
    let instance = |name: u16, wght: i32| InstanceRecord {
        subfamily_name_id: NameId::new(name),
        flags: 0,
        coordinates: vec![Fixed::from_i32(wght)],
        post_script_name_id: None,
    };
    let instances = vec![instance(257, 400), instance(258, 550), instance(259, 700), instance(260, 600), instance(261, 650)];
    let fvar = Fvar::new(AxisInstanceArrays::new(vec![axis], instances));
    let name = name_table(&[
        (1, VARIABLE_FAMILY),
        (2, "Regular"),
        (4, "Varitest Sans Regular"),
        (6, "VaritestSans-Regular"),
        (256, "Weight"),
        (257, "Normal"),
        (258, "SemiBold"),
        (259, "Bold"),
        (261, "BOLD"),
    ])?;
    let mut builder = FontBuilder::new();
    builder.add_raw(Tag::new(b"name"), name);
    builder.add_table(&fvar).ok()?.add_table(&gvar).ok()?;
    // The rest of Noto Sans's tables as they are.
    for r in font.table_directory.table_records() {
        let tag = Tag::new(&r.tag().to_be_bytes());
        if !builder.contains(tag) {
            builder.add_raw(tag, font.table_data(r.tag())?.as_bytes());
        }
    }
    Some(builder.build())
}

/// A `name` table of Windows English (US) records (name id, string).
pub fn name_table(records: &[(u16, &str)]) -> Option<Vec<u8>> {
    let mut records = records.to_vec();
    records.sort_by_key(|r| r.0);
    let n = u16::try_from(records.len()).ok()?;
    let (mut head, mut strings) = (Vec::new(), Vec::new());
    for v in [0, n, 6 + 12 * n] {
        head.extend_from_slice(&v.to_be_bytes());
    }
    for (id, s) in records {
        let bytes: Vec<u8> = s.encode_utf16().flat_map(u16::to_be_bytes).collect();
        for v in [3, 1, 0x0409, id, u16::try_from(bytes.len()).ok()?, u16::try_from(strings.len()).ok()?] {
            head.extend_from_slice(&v.to_be_bytes());
        }
        strings.extend(bytes);
    }
    head.extend(strings);
    Some(head)
}

/// The family of [`vertical_font`].
pub const VERTICAL_FAMILY: &str = "Vertitest Sans";

/// The character whose vertical metrics [`vertical_font`] sets apart: 1.4 em down the column, its
/// vertical origin 1.1 em above the baseline.
pub const VERTICAL_TALL: char = '§';

/// The bundled Noto Sans Regular as family [`VERTICAL_FAMILY`] with vertical metrics (`vhea`,
/// `vmtx`, and `VORG` when `vorg`): every glyph one em down the column with its vertical origin at
/// the usual 0.88 em, except [`VERTICAL_TALL`] (1.4 em, origin 1.1 em). Without `VORG` the origin
/// comes from each glyph's top and top side bearing, as in TrueType fonts. `None` when it can't be
/// built.
pub fn vertical_font(vorg: bool) -> Option<Vec<u8>> {
    const BASE: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");
    let font = skrifa::FontRef::new(BASE).ok()?;
    let n = font.maxp().ok()?.num_glyphs();
    let metrics = font.glyph_metrics(Size::unscaled(), LocationRef::default());
    let tall = font.charmap().map(VERTICAL_TALL)?;
    let cell = |g: GlyphId| -> (u16, i16) { if g == tall { (1400, 1100) } else { (1000, 880) } };
    let be16 = |v: &mut Vec<u8>, x: i16| v.extend_from_slice(&x.to_be_bytes());
    // vmtx: (advance height, top side bearing = origin − the glyph's top) for every glyph.
    let mut vmtx = Vec::with_capacity(4 * usize::from(n));
    for g in 0..u32::from(n) {
        let id = GlyphId::new(g);
        let (advance, origin) = cell(id);
        let top = metrics.bounds(id).map_or(0.0, |b| b.y_max).round() as i16;
        vmtx.extend_from_slice(&advance.to_be_bytes());
        be16(&mut vmtx, origin - top);
    }
    // vhea 1.1: ascent/descent of the em box across the column, then the metrics' count.
    let mut vhea = Vec::with_capacity(36);
    vhea.extend_from_slice(&0x0001_1000_u32.to_be_bytes());
    for x in [500, -500, 0] {
        be16(&mut vhea, x);
    }
    vhea.extend_from_slice(&1400_u16.to_be_bytes());
    for x in [0, 0, 1400, 1, 0, 0, 0, 0, 0, 0, 0] {
        be16(&mut vhea, x);
    }
    vhea.extend_from_slice(&n.to_be_bytes());
    let name = name_table(&[(1, VERTICAL_FAMILY), (2, "Regular"), (4, "Vertitest Sans Regular"), (6, "VertitestSans-Regular")])?;
    let mut builder = FontBuilder::new();
    builder.add_raw(Tag::new(b"name"), name);
    builder.add_raw(Tag::new(b"vhea"), vhea);
    builder.add_raw(Tag::new(b"vmtx"), vmtx);
    if vorg {
        // VORG 1.0: the default origin, then the one glyph that differs.
        let mut t = Vec::new();
        for x in [1, 0, 880, 1] {
            be16(&mut t, x);
        }
        t.extend_from_slice(&u16::try_from(tall.to_u32()).ok()?.to_be_bytes());
        be16(&mut t, 1100);
        builder.add_raw(Tag::new(b"VORG"), t);
    }
    for r in font.table_directory.table_records() {
        let tag = Tag::new(&r.tag().to_be_bytes());
        if !builder.contains(tag) {
            builder.add_raw(tag, font.table_data(r.tag())?.as_bytes());
        }
    }
    Some(builder.build())
}

/// The family of [`twin_font`]: two versions of one family and style.
pub const TWIN_FAMILY: &str = "Twintest Sans";

/// The version strings of [`twin_font`]'s older and newer face.
pub const TWIN_VERSIONS: [&str; 2] = ["Version 1.000", "Version 2.000"];

/// A face of family [`TWIN_FAMILY`], style Regular, in version [`TWIN_VERSIONS`]`[0]` (the bundled
/// Noto Sans Regular) or, `newer`, `[1]` (the bundled Inter Regular): one name, two sets of
/// glyphs, as a font library that keeps an old version beside a new one has. `None` when it can't
/// be built.
pub fn twin_font(newer: bool) -> Option<Vec<u8>> {
    const OLDER: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");
    const NEWER: &[u8] = include_bytes!("../../../assets/fonts/Inter-Regular.ttf");
    let font = skrifa::FontRef::new(if newer { NEWER } else { OLDER }).ok()?;
    let version = TWIN_VERSIONS[usize::from(newer)];
    let name = name_table(&[(1, TWIN_FAMILY), (2, "Regular"), (4, "Twintest Sans Regular"), (5, version), (6, "TwintestSans-Regular")])?;
    let mut builder = FontBuilder::new();
    builder.add_raw(Tag::new(b"name"), name);
    for r in font.table_directory.table_records() {
        let tag = Tag::new(&r.tag().to_be_bytes());
        if !builder.contains(tag) {
            builder.add_raw(tag, font.table_data(r.tag())?.as_bytes());
        }
    }
    Some(builder.build())
}

#[cfg(test)]
mod rename_tests {
    use super::*;
    use crate::FontDb;

    const SANS: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");

    #[test]
    fn renamed_family_names_can_shrink_grow_or_contain_unicode() {
        for family in ["X", "A Much Longer Fixture Sans Family", "検証用サンセリフ"] {
            let data = renamed_font(SANS, &[("Noto Sans", family)]).unwrap();
            let db = FontDb::with_font_dirs(vec![]);
            assert_eq!(db.add_font(data), 1);
            let face = db.face(family, "Regular").unwrap();
            assert_eq!(face.family, family);
            assert_eq!(face.style, "Regular");
            assert!(face.covers('A') && face.covers('é'));
        }
    }

    #[test]
    fn renamed_postscript_name_is_found_without_changing_glyph_metrics() {
        let data = renamed_font(SANS, &[("Noto Sans", "Fixture Sans"), ("NotoSans-Regular", "FixtureSans-Regular")]).unwrap();
        let db = FontDb::with_font_dirs(vec![]);
        assert_eq!(db.add_font(data), 1);
        let original = db.face("Noto Sans", "Regular").unwrap();
        let fixture = db.find_postscript("FixtureSans-Regular").unwrap();
        assert_eq!(fixture.family, "Fixture Sans");
        assert_eq!(fixture.units_per_em(), original.units_per_em());
        for c in "AVfilé".chars() {
            assert_eq!(fixture.glyph_for(c), original.glyph_for(c));
            assert_eq!(fixture.advance(fixture.glyph_for(c)), original.advance(original.glyph_for(c)));
        }
    }

    #[test]
    fn renamed_font_rejects_invalid_or_unrepresentable_name_tables() {
        assert!(renamed_font(&[0; 16], &[("Noto Sans", "Fixture")]).is_none());
        assert!(renamed_font(SANS, &[("Noto Sans", &"x".repeat(65_536))]).is_none());
    }
}
