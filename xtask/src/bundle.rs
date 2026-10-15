//! `cargo xtask bundle`: build the release app and wrap it as `dist/MIXEL Vector.app` (macOS), with the
//! committed app icon `assets/app-icon/vectorcraft.icns` (regenerate it with `packaging/icons.sh`).

use std::path::Path;
use std::process::Command;

pub fn run(root: &Path) -> Result<(), String> {
    let status = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .current_dir(root)
        .args(["build", "--release", "-p", "vectorcraft", "-p", "vectorcraft-cli"])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("release build failed".into());
    }
    let target = std::env::var("CARGO_TARGET_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| root.join("target"));
    let app = root.join("dist/MIXEL Vector.app/Contents");
    let _ = std::fs::remove_dir_all(root.join("dist/MIXEL Vector.app"));
    std::fs::create_dir_all(app.join("MacOS")).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(app.join("Resources")).map_err(|e| e.to_string())?;
    std::fs::copy(target.join("release/vectorcraft"), app.join("MacOS/MIXEL Vector")).map_err(|e| format!("copy app: {e}"))?;
    std::fs::copy(target.join("release/vectorcraft-cli"), app.join("MacOS/vectorcraft-cli")).map_err(|e| format!("copy cli: {e}"))?;
    std::fs::copy(root.join("assets/app-icon/vectorcraft.icns"), app.join("Resources/MIXEL Vector.icns")).map_err(|e| format!("copy icon: {e}"))?;
    let sha = std::env::var("VECTORCRAFT_BUILD_SHA").unwrap_or_else(|_| "unknown".into());
    std::fs::write(app.join("Info.plist"), info_plist(env!("CARGO_PKG_VERSION"), &sha)).map_err(|e| e.to_string())?;
    println!("built {}", root.join("dist/MIXEL Vector.app").display());
    Ok(())
}

/// The release `Info.plist` template, which `packaging/macos/package.sh` fills in the same way: the
/// development bundle declares the same document types (Finder opens them with the app).
const INFO_PLIST: &str = include_str!("../../packaging/macos/Info.plist.in");

/// The bundle's `Info.plist`: [`INFO_PLIST`] for `version` and the build commit `sha`.
fn info_plist(version: &str, sha: &str) -> String {
    let short = version.split('-').next().unwrap_or(version);
    INFO_PLIST.replace("@VERSION@", version).replace("@SHORT_VERSION@", short).replace("@BUILD_SHA@", sha)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// File types and wording a raster image editor's packaging carries (lowercase).
    const RASTER_EDITOR: &[&str] = &["pcraft", "psd", "psb", "qoi", "photo", "rastergraphics", "image editor"];

    // These exact sources are shared by all seven products. Bind their packaging-relative paths
    // and reviewed content, not a directory, file type or basename. Git checkout line endings may differ.
    const SHARED_PACKAGING: &[(&str, &str, &str)] = &[
        (
            "macos/mas_package.py",
            "6ca3103d3247610b1c89f319fbdbcc44c92c69b9a52f41dea1d97b7efeeeef92",
            include_str!("../../packaging/macos/mas_package.py"),
        ),
        (
            "macos/README-mas.md",
            "c7f65dce62a9c0df908d69e6569ba01ae88bd7269e00af807cdc5a77b9aedc60",
            include_str!("../../packaging/macos/README-mas.md"),
        ),
    ];

    fn packaging_brand_error(relative: &Path, text: &str) -> Option<String> {
        for &(path, expected, _) in SHARED_PACKAGING {
            if relative == Path::new(path) {
                let canonical = text.replace("\r\n", "\n");
                return (crate::sha256::hex(canonical.as_bytes()) != expected)
                    .then(|| "shared packaging source differs from its reviewed content pin".to_string());
            }
        }
        let lowercase = text.to_ascii_lowercase();
        RASTER_EDITOR.iter().find(|word| lowercase.contains(**word)).map(|word| format!("mentions forbidden term {word}"))
    }

    fn text_files(dir: &Path, out: &mut Vec<(String, String)>) {
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                text_files(&p, out);
            } else if let Ok(t) = std::fs::read_to_string(&p) {
                out.push((p.display().to_string(), t));
            }
        }
    }

    #[test]
    fn packaging_describes_a_vector_app() {
        let packaging = crate::root().join("packaging");
        let mut files = vec![];
        text_files(&packaging, &mut files);
        assert!(files.len() > 5, "packaging files found");
        for &(relative, _, _) in SHARED_PACKAGING {
            let expected = packaging.join(relative);
            assert_eq!(files.iter().filter(|(path, _)| Path::new(path) == expected.as_path()).count(), 1, "required shared source: {relative}");
        }
        for (path, text) in &files {
            let relative = Path::new(path).strip_prefix(&packaging).unwrap();
            let error = packaging_brand_error(relative, text);
            assert!(error.is_none(), "{path}: {error:?}");
        }
        let read = |p: &str| std::fs::read_to_string(crate::root().join(p)).unwrap();
        let plist = read("packaging/macos/Info.plist.in");
        assert!(plist.contains("<array><string>vectorcraft</string><string>drawcraft</string></array>"), "native format extensions");
        assert!(read("packaging/linux/ch.mixel.vector.desktop").contains("Categories=Graphics;2DGraphics;VectorGraphics;"));
        assert!(read("packaging/linux/ch.mixel.vector.metainfo.xml.in").contains("<category>VectorGraphics</category>"));
    }

    #[test]
    fn reviewed_shared_packaging_accepts_lf_and_crlf() {
        for &(path, _, text) in SHARED_PACKAGING {
            let lf = text.replace("\r\n", "\n");
            assert_eq!(packaging_brand_error(Path::new(path), &lf), None, "{path}: LF");
            let crlf = lf.replace('\n', "\r\n");
            assert_eq!(packaging_brand_error(Path::new(path), &crlf), None, "{path}: CRLF");
        }
    }

    #[test]
    fn changed_shared_packaging_rejects_the_source_exception() {
        for &(path, _, text) in SHARED_PACKAGING {
            let changed = format!("{text}\n# additional source text\n");
            let error = packaging_brand_error(Path::new(path), &changed).unwrap();
            assert!(error.contains("reviewed content pin"), "{path}: {error}");
            assert!(packaging_brand_error(Path::new(path), "MIXEL Photo").is_some(), "{path}: replacement content");
        }
    }

    #[test]
    fn shared_packaging_at_other_paths_is_still_brand_checked() {
        for &(path, _, text) in SHARED_PACKAGING {
            let moved = Path::new("unreviewed").join(path);
            assert!(packaging_brand_error(&moved, text).is_some(), "same basename at {moved:?}");
            let sibling = Path::new(path).with_file_name("unreviewed.txt");
            assert!(packaging_brand_error(&sibling, text).is_some(), "same directory at {sibling:?}");
        }
    }

    #[test]
    fn product_packaging_rejects_every_raster_editor_term() {
        for path in ["macos/Info.plist.in", "linux/ch.mixel.vector.desktop", "linux/ch.mixel.vector.metainfo.xml.in"] {
            assert_eq!(packaging_brand_error(Path::new(path), "MIXEL Vector"), None, "{path}: vector control");
            for word in RASTER_EDITOR {
                assert!(packaging_brand_error(Path::new(path), &format!("MIXEL Vector {word}")).is_some(), "{path}: {word}");
                assert!(
                    packaging_brand_error(Path::new(path), &format!("MIXEL Vector {}", word.to_ascii_uppercase())).is_some(),
                    "{path}: uppercase {word}"
                );
            }
        }
    }

    #[test]
    fn dev_bundle_plist_is_the_release_one() {
        let p = info_plist("1.2.3-rc.1", "abc123");
        assert!(p.contains("<string>1.2.3</string>") && p.contains("<string>1.2.3-rc.1</string>"), "{p}");
        assert!(p.contains("<string>abc123</string>"));
        assert!(!p.contains('@'), "every placeholder filled");
        assert!(p.contains("<key>CFBundleDocumentTypes</key>") && p.contains("<string>pdf</string>"), "Finder opens files with it");
    }
}
