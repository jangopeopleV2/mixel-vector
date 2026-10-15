//! Complete output-set checks. A selected file never authorizes generated siblings.
use std::collections::BTreeSet;
use std::path::{Component, Path};

pub const FOLDER_REQUIRED: &str = "Choose the containing folder for all output files before saving. No files were written.";
pub const WRONG_FOLDER: &str = "Choose a folder that contains every output file. No files were written.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputAccess {
    pub requested: String,
    pub paths: Vec<String>,
    pub missing_folders: Vec<String>,
}
impl OutputAccess {
    pub fn require_complete(&self) -> Result<(), String> {
        if self.missing_folders.is_empty() { Ok(()) } else { Err(FOLDER_REQUIRED.into()) }
    }
    pub fn selected_folder_covers(&self, selected: &Path) -> bool {
        valid(selected) && self.missing_folders.iter().all(|p| Path::new(p).starts_with(selected))
    }
}
fn valid(path: &Path) -> bool {
    path.is_absolute() && !path.components().any(|c| matches!(c, Component::ParentDir))
}

/// The same pure policy used by native preflight and host-service flow tests.
/// The predicates describe existing access; this function never creates a grant.
pub fn output_access_with(
    requested: &str,
    paths: &[String],
    mut resolve: impl FnMut(&str) -> Result<String, String>,
    granted: impl Fn(&Path) -> bool,
) -> Result<OutputAccess, String> {
    let requested = resolve(requested)?;
    let paths: Vec<String> = paths.iter().map(|p| resolve(p)).collect::<Result<_, _>>()?;
    if !valid(Path::new(&requested)) || paths.iter().any(|p| !valid(Path::new(p))) {
        return Err("Choose an absolute save destination".into());
    }
    if paths.is_empty() {
        return Err(FOLDER_REQUIRED.into());
    }
    let mut missing = BTreeSet::new();
    if paths.len() != 1 || paths.first() != Some(&requested) {
        for path in &paths {
            let folder = Path::new(path).parent().ok_or(FOLDER_REQUIRED)?;
            if !granted(folder) {
                missing.insert(folder.to_string_lossy().into_owned());
            }
        }
    }
    Ok(OutputAccess { requested, paths, missing_folders: missing.into_iter().collect() })
}

/// Directory fan-out must acquire its explicit scope before even creating the destination.
/// The injected operation keeps the denied-before-I/O boundary directly testable.
#[cfg(any(not(target_arch = "wasm32"), test))]
pub(super) fn create_scoped_directory(
    path: &Path,
    granted: impl FnOnce(&Path) -> bool,
    create: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<(), String> {
    if !valid(path) || !granted(path) {
        return Err(FOLDER_REQUIRED.into());
    }
    create(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// Resolve moved destinations, then verify the ENTIRE set before any output writer runs.
/// Ordinary frontends retain their existing path and permission behavior.
pub fn prepare_output_access(requested: &str, paths: &[String]) -> Result<OutputAccess, String> {
    if !crate::macos_mas::enabled() {
        return Ok(OutputAccess { requested: requested.into(), paths: paths.to_vec(), missing_folders: vec![] });
    }
    output_access_with(
        requested,
        paths,
        |p| crate::macos_mas::resolve(p).map(|p| p.to_string_lossy().into_owned()),
        |p| crate::macos_mas::directory_granted(p),
    )
}

#[cfg(test)]
mod tests {
    use super::super::{Encoded, SaveMode, deferred_save_job, encode_all, save_plan};
    use super::*;
    use crate::Session;
    use serde_json::json;
    use std::sync::Arc;

    fn path(name: &str) -> String {
        std::env::current_dir().unwrap().join("output-access-fixture").join(name).to_str().unwrap().to_owned()
    }
    fn access(target: &str, files: &[String], granted: impl Fn(&Path) -> bool) -> OutputAccess {
        output_access_with(target, files, |p| Ok(p.to_owned()), granted).unwrap()
    }
    #[test]
    fn selected_single_file_needs_no_inferred_parent_scope() {
        let target = path("selected.svg");
        let checked = access(&target, std::slice::from_ref(&target), |_| false);
        assert!(checked.require_complete().is_ok());
    }
    #[test]
    fn renamed_artboard_sibling_requires_a_directory_even_when_alone() {
        let target = path("set.svg");
        let checked = access(&target, &[path("set-Artboard-1.svg")], |_| false);
        assert_eq!(checked.missing_folders, [Path::new(&target).parent().unwrap().to_str().unwrap()]);
        assert_eq!(checked.require_complete().unwrap_err(), FOLDER_REQUIRED);
    }
    #[test]
    fn linked_svg_and_image_map_targets_are_part_of_the_same_preflight() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"width": 20, "height": 20})).unwrap();
        s.execute("shape.rectangle", &json!({"x": 1, "y": 1, "width": 5, "height": 5})).unwrap();
        s.execute("attributes.set", &json!({"url": "https://example.com", "imageMap": "rectangle"})).unwrap();
        let doc = &s.active().unwrap().doc;
        let svg = Encoded {
            files: vec![(None, b"<svg/>".to_vec())],
            linked: vec![vectorcraft_svg::LinkedImage { name: "image.png".into(), bytes: Arc::new(vec![1]) }],
            ..Default::default()
        };
        let target = path("linked.svg");
        let files = svg.named(doc, &target).into_iter().map(|(p, _)| p).collect::<Vec<_>>();
        assert_eq!(files, [target.clone(), path("image.png")]);
        assert!(access(&target, &files, |_| false).require_complete().is_err());
        let target = path("map.jpg");
        let encoded = encode_all(doc, "jpg", &json!({"imageMap": "client", "useArtboards": false})).unwrap();
        let files = encoded.named(doc, &target).into_iter().map(|(p, _)| p).collect::<Vec<_>>();
        assert_eq!(files, [target.clone(), path("map.html")]);
        assert!(access(&target, &files, |_| false).require_complete().is_err());
        assert!(access(&target, &files, |p| p == Path::new(&target).parent().unwrap()).require_complete().is_ok());
    }
    #[test]
    fn moved_targets_are_resolved_before_checking_existing_directory_access() {
        let old = path("old");
        let new = path("new");
        let target = format!("{old}/set.svg");
        let files = [format!("{old}/set-Artboard-1.svg"), format!("{old}/set-Artboard-2.svg")];
        let checked = output_access_with(&target, &files, |p| Ok(p.replacen(&old, &new, 1)), |p| p == Path::new(&new)).unwrap();
        assert_eq!(checked.requested, format!("{new}/set.svg"));
        assert!(checked.paths.iter().all(|p| Path::new(p).starts_with(&new)));
        assert!(checked.require_complete().is_ok());
    }
    #[test]
    fn unrelated_folder_and_parent_traversal_do_not_authorize_outputs() {
        let target = path("chosen/set.svg");
        let checked = access(&target, &[path("chosen/set-1.svg"), path("chosen/set-2.svg")], |_| false);
        assert!(!checked.selected_folder_covers(Path::new(&path("unrelated"))));
        assert!(checked.selected_folder_covers(Path::new(&path("chosen"))));
        assert!(output_access_with(&target, &[path("chosen/../escape.svg")], |p| Ok(p.to_owned()), |_| true).is_err());
    }
    #[test]
    fn screens_directory_denial_happens_before_any_creation_or_output() {
        let folder = std::path::PathBuf::from(path("screens"));
        let called = std::cell::Cell::new(false);
        let result = create_scoped_directory(
            &folder,
            |_| false,
            |_| {
                called.set(true);
                Ok(())
            },
        );
        assert_eq!(result, Err(FOLDER_REQUIRED.into()));
        assert!(!called.get(), "a denied remembered folder cannot start any filesystem operation");
        create_scoped_directory(
            &folder,
            |p| p == folder.as_path(),
            |p| {
                assert_eq!(p, folder.as_path());
                called.set(true);
                Ok(())
            },
        )
        .unwrap();
        assert!(called.get(), "the same production boundary permits an explicit existing directory scope");
    }
    #[test]
    fn deferred_save_does_not_change_dates_or_dirty_state_until_completion() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"artboards": 2})).unwrap();
        s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 5, "height": 5})).unwrap();
        let original = s.active().unwrap().doc.clone();
        let uid = s.active().unwrap().uid;
        let plan = save_plan(&s, SaveMode::SaveAs, &json!({"path": path("saved.vectorcraft"), "options": {"separateArtboards": true}})).unwrap();
        let job = deferred_save_job(&mut s, plan).unwrap();
        assert!(Arc::ptr_eq(&original, &s.active().unwrap().doc));
        assert!(s.active().unwrap().is_dirty());
        let encoded = job.encode().unwrap();
        assert_eq!(job.output_paths(&encoded).len(), 3);
        drop(encoded);
        job.complete(&mut s, uid);
        assert!(!s.active().unwrap().is_dirty());
        assert_eq!(s.active().unwrap().path.as_deref(), Some(path("saved.vectorcraft").as_str()));
    }
}
