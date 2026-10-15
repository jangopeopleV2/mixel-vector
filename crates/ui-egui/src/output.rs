//! A prepared output can wait for explicit folder consent without encoding or picking its file again.
use crate::{VectorcraftApp, background};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex, PoisonError};
use vectorcraft_engine::cmd::fileio::{self, Encoded, OutputAccess, SaveJob};

pub(crate) type Policy = Arc<dyn Fn(&str, &[String]) -> Result<OutputAccess, String> + Send + Sync>;
pub(crate) type Prepare = Box<dyn FnOnce() -> Result<Prepared, String> + Send>;
pub(crate) type Ready = Box<dyn FnOnce(&mut VectorcraftApp, Value) -> Result<Value, String>>;

pub(crate) fn native_policy() -> Option<Policy> {
    if !crate::macos_mas::enabled() {
        return None;
    }
    let policy: Policy = Arc::new(fileio::prepare_output_access);
    Some(policy)
}

pub(crate) enum Prepared {
    Save { job: SaveJob, encoded: Encoded },
    Export { doc: Arc<vectorcraft_doc::Document>, target: String, encoded: Encoded },
}
impl Prepared {
    fn target(&self) -> &str {
        match self {
            Self::Save { job, .. } => job.plan().path.as_deref().unwrap_or(&job.plan().name),
            Self::Export { target, .. } => target,
        }
    }
    fn paths(&self) -> Vec<String> {
        match self {
            Self::Save { job, encoded } => job.output_paths(encoded),
            Self::Export { doc, target, encoded } => encoded.named(doc, target).into_iter().map(|(p, _)| p).collect(),
        }
    }
    fn write(&self, write: background::Writer) -> Result<Value, String> {
        match self {
            Self::Save { job, encoded } => {
                job.write_prepared(encoded, |p, b| write(p, b).map_err(vectorcraft_engine::EngineError::Other)).map_err(|e| e.to_string())
            }
            Self::Export { doc, target, encoded } => {
                let files = crate::io::write_encoded(write, doc, target, encoded)?;
                let mut out = json!({ "path": files.first().unwrap_or(target), "warnings": &encoded.warnings });
                if files.len() > 1 {
                    out["files"] = json!(files);
                }
                Ok(out)
            }
        }
    }
}

/// The only retry is of retained encoded bytes after a real explicit folder selection.
/// Boxed continuations keep recursive resume calls at one concrete Rust type.
pub(crate) fn run(
    app: &mut VectorcraftApp,
    enabled: bool,
    label: String,
    uid: Option<u64>,
    policy: Policy,
    prepare: Prepare,
    ready: Ready,
) -> Result<Value, String> {
    let pending = Arc::new(Mutex::new(None::<(Prepared, OutputAccess)>));
    let worker_pending = pending.clone();
    let worker_policy = policy.clone();
    let next_label = label.clone();
    background::run(
        app,
        enabled,
        label,
        uid,
        move |write| {
            let prepared = prepare()?;
            let access = worker_policy(prepared.target(), &prepared.paths())?;
            if !access.missing_folders.is_empty() {
                *worker_pending.lock().unwrap_or_else(PoisonError::into_inner) = Some((prepared, access));
                return Ok(json!({ "outputFolderPending": true }));
            }
            prepared.write(write)
        },
        move |app, result| {
            let result = result?;
            if result["outputFolderPending"].as_bool() != Some(true) {
                return ready(app, result);
            }
            let (prepared, access) = pending.lock().unwrap_or_else(PoisonError::into_inner).take().ok_or("no data")?;
            app.status(fileio::FOLDER_REQUIRED.to_owned());
            crate::picks::folder_once(app, uid, move |app, selected| {
                let selected = crate::macos_mas::resolve(selected)?;
                let missing = access.missing_folders.iter().map(crate::macos_mas::resolve).collect::<Result<Vec<_>, _>>()?;
                if !missing.iter().all(|p| p.starts_with(&selected)) {
                    return Err(fileio::WRONG_FOLDER.into());
                }
                let refreshed = policy(prepared.target(), &prepared.paths())?;
                if !refreshed.selected_folder_covers(&selected) || refreshed.require_complete().is_err() {
                    return Err(fileio::WRONG_FOLDER.into());
                }
                run(app, enabled, next_label, uid, policy, Box::new(move || Ok(prepared)), ready)
            })
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Services;
    use crate::picks::{self, PickRequest};
    use crate::tests_picks::{Shown, off_the_ui_thread};
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use vectorcraft_engine::Session;

    type Writes = Rc<RefCell<Vec<(String, Vec<u8>)>>>;
    type Grants = Arc<Mutex<Vec<PathBuf>>>;

    fn path(name: &str) -> String {
        std::env::current_dir().unwrap().join("vector-output-flow-fixture").join(name).to_str().unwrap().to_owned()
    }
    type Fixture = (VectorcraftApp, Shown, Writes, Grants, Rc<RefCell<Vec<PickRequest>>>);

    fn fixture() -> Fixture {
        let mut services = Services { dialogs_are_sheets: true, ..Default::default() };
        let (shown, _) = off_the_ui_thread(&mut services);
        let history = Rc::new(RefCell::new(vec![]));
        let history_copy = history.clone();
        let mut start = services.start_pick.take().unwrap();
        services.start_pick = Some(Box::new(move |request| {
            history_copy.borrow_mut().push(request.clone());
            start(request)
        }));
        let writes = Writes::default();
        let writes_copy = writes.clone();
        services.write = Some(Box::new(move |p, b| {
            writes_copy.borrow_mut().push((p.to_owned(), b.to_vec()));
            Ok(())
        }));
        let grants = Grants::default();
        let grants_copy = grants.clone();
        let policy: Policy = Arc::new(move |target, paths| {
            fileio::output_access_with(
                target,
                paths,
                |p| Ok(p.to_owned()),
                |p| grants_copy.lock().unwrap().iter().any(|folder| p.starts_with(folder)),
            )
        });
        let mut app = VectorcraftApp::new(Session::new(), services);
        app.output_policy = Some(policy);
        app.session.prefs.background_save = false;
        app.session.prefs.background_export = false;
        app.run("file.new", json!({"width": 120, "height": 80, "artboards": 2})).unwrap();
        app.run("shape.rectangle", json!({"x": 10, "y": 10, "width": 40, "height": 30})).unwrap();
        (app, shown, writes, grants, history)
    }
    fn poll(app: &mut VectorcraftApp) {
        picks::poll(app, &egui::Context::default());
    }
    fn grant(shown: &Shown, grants: &Grants, folder: &str) {
        grants.lock().unwrap().push(PathBuf::from(folder));
        shown.answer(&[folder]);
    }
    fn names(writes: &Writes) -> Vec<String> {
        writes.borrow().iter().map(|(p, _)| p.clone()).collect()
    }
    fn save_separate(app: &mut VectorcraftApp, shown: &Shown, target: &str) {
        assert!(app.run("file.saveAs", json!({"format": "vectorcraft", "options": {"separateArtboards": true}})).is_err());
        assert!(matches!(shown.requests().as_slice(), [PickRequest::Save(_)]));
        shown.answer(&[target]);
        poll(app);
    }

    #[test]
    fn two_artboard_svg_keeps_options_and_original_document_across_both_sheets() {
        let (mut app, shown, writes, grants, history) = fixture();
        let uid = app.session.active().unwrap().uid;
        let original = app.session.active().unwrap().doc.clone();
        app.run("file.export.svg", Value::Null).unwrap();
        app.ui.dialog.as_mut().unwrap().fields.insert("decimals".into(), json!(1));
        assert!(crate::dialogs::confirm(&mut app).is_err());
        assert!(matches!(shown.requests().as_slice(), [PickRequest::Save(_)]));
        shown.answer(&[&path("set.svg")]);
        poll(&mut app);
        assert_eq!(shown.requests(), [PickRequest::Folder]);
        assert!(writes.borrow().is_empty(), "even the first artboard waits for the complete grant");
        app.session.execute("file.new", &json!({"width": 200, "height": 160})).unwrap();
        let other = app.session.active().unwrap().uid;
        grant(&shown, &grants, &path(""));
        poll(&mut app);
        assert_eq!(app.session.active().unwrap().uid, uid);
        assert_ne!(uid, other);
        if crate::macos_mas::enabled() {
            assert!(writes.borrow().is_empty(), "logical UI access cannot replace the real native registry");
            assert!(app.ui.status.contains(fileio::FOLDER_REQUIRED), "{}", app.ui.status);
            let st = app.session.active().unwrap();
            assert!(st.path.is_none() && st.is_dirty());
            assert!(Arc::ptr_eq(&original, &st.doc));
        } else {
            assert_eq!(names(&writes), [path("set-Artboard-1.svg"), path("set-Artboard-2.svg")]);
        }
        assert_eq!(app.ui.svg_options["decimals"], 1);
        assert!(matches!(history.borrow().as_slice(), [PickRequest::Save(_), PickRequest::Folder]));
        assert!(!app.file_dialog_open(), "the Save picker is never replayed after the folder picker");
        assert!(app.session.documents().iter().find(|d| d.uid == other).unwrap().path.is_none());
    }

    #[test]
    fn cancelling_separate_save_keeps_dates_path_and_dirty_document() {
        let (mut app, shown, writes, _, _) = fixture();
        let original = app.session.active().unwrap().doc.clone();
        save_separate(&mut app, &shown, &path("set.vectorcraft"));
        assert_eq!(shown.requests(), [PickRequest::Folder]);
        assert!(writes.borrow().is_empty());
        assert!(Arc::ptr_eq(&original, &app.session.active().unwrap().doc));
        shown.answer(&[]);
        poll(&mut app);
        let st = app.session.active().unwrap();
        assert!(writes.borrow().is_empty());
        assert!(st.path.is_none() && st.is_dirty());
        assert!(Arc::ptr_eq(&original, &st.doc));
        assert!(!app.file_dialog_open());
    }

    #[test]
    fn an_unrelated_folder_never_writes_or_marks_a_save_complete() {
        let (mut app, shown, writes, grants, _) = fixture();
        let original = app.session.active().unwrap().doc.clone();
        save_separate(&mut app, &shown, &path("chosen/set.vectorcraft"));
        grant(&shown, &grants, &path("other"));
        poll(&mut app);
        let st = app.session.active().unwrap();
        assert!(writes.borrow().is_empty());
        assert!(st.path.is_none() && st.is_dirty());
        assert!(Arc::ptr_eq(&original, &st.doc));
        assert!(app.ui.status.contains(fileio::WRONG_FOLDER), "{}", app.ui.status);
    }

    #[test]
    fn choosing_the_right_folder_without_retained_access_still_writes_nothing() {
        let (mut app, shown, writes, _, _) = fixture();
        save_separate(&mut app, &shown, &path("ungranted.vectorcraft"));
        shown.answer(&[&path("")]);
        poll(&mut app);
        assert!(writes.borrow().is_empty());
        assert!(app.session.active().unwrap().path.is_none());
        assert!(app.session.active().unwrap().is_dirty());
        assert!(app.ui.status.contains(fileio::WRONG_FOLDER), "{}", app.ui.status);
    }

    #[test]
    fn folder_start_failure_never_retries_the_save_picker_or_writes() {
        let (mut app, shown, writes, _, history) = fixture();
        assert!(app.run("file.saveAs", json!({"options": {"separateArtboards": true}})).is_err());
        app.services.start_pick = None;
        shown.answer(&[&path("unavailable.vectorcraft")]);
        poll(&mut app);
        assert!(writes.borrow().is_empty());
        assert!(app.session.active().unwrap().path.is_none());
        assert!(app.session.active().unwrap().is_dirty());
        assert!(app.ui.status.contains("native file dialog service is unavailable"), "{}", app.ui.status);
        assert!(matches!(history.borrow().as_slice(), [PickRequest::Save(_)]));
        assert!(!app.file_dialog_open());
    }

    #[test]
    fn an_existing_directory_grant_saves_all_artboards_without_another_picker() {
        let (mut app, shown, writes, grants, history) = fixture();
        let original = app.session.active().unwrap().doc.clone();
        grants.lock().unwrap().push(PathBuf::from(path("")));
        save_separate(&mut app, &shown, &path("set.vectorcraft"));
        assert!(shown.requests().is_empty());
        assert!(matches!(history.borrow().as_slice(), [PickRequest::Save(_)]));
        let st = app.session.active().unwrap();
        if crate::macos_mas::enabled() {
            assert!(writes.borrow().is_empty(), "logical grants cannot authorize the real native writer");
            assert!(app.ui.status.contains(fileio::FOLDER_REQUIRED), "{}", app.ui.status);
            assert!(st.path.is_none() && st.is_dirty());
            assert!(Arc::ptr_eq(&original, &st.doc));
        } else {
            assert_eq!(names(&writes), [path("set.vectorcraft"), path("set-Artboard-1.vectorcraft"), path("set-Artboard-2.vectorcraft")]);
            assert_eq!(st.path.as_deref(), Some(path("set.vectorcraft").as_str()));
            assert!(!st.is_dirty());
        }
    }

    #[test]
    fn a_single_native_save_keeps_its_streaming_path_and_needs_no_parent_grant() {
        let (mut app, shown, writes, _, history) = fixture();
        let streams = Arc::new(Mutex::new(Vec::new()));
        let streams_copy = streams.clone();
        app.services.stream_shared = Some(Arc::new(move |p, fill| {
            let mut bytes = vec![];
            fill(&mut bytes).map_err(|e| e.to_string())?;
            streams_copy.lock().unwrap().push((p.to_owned(), bytes.len()));
            Ok(())
        }));
        assert!(app.run("file.saveAs", json!({"options": {}})).is_err());
        shown.answer(&[&path("single.vectorcraft")]);
        poll(&mut app);
        assert!(shown.requests().is_empty());
        assert!(writes.borrow().is_empty(), "native single-file saves still stream");
        assert_eq!(streams.lock().unwrap().len(), 1);
        assert_eq!(streams.lock().unwrap()[0].0, path("single.vectorcraft"));
        assert!(streams.lock().unwrap()[0].1 > 0);
        assert!(!app.session.active().unwrap().is_dirty());
        assert!(matches!(history.borrow().as_slice(), [PickRequest::Save(_)]));
    }

    #[test]
    fn image_map_companion_waits_before_the_main_image_is_written() {
        let (mut app, shown, writes, grants, _) = fixture();
        app.run("attributes.set", json!({"url": "https://example.com", "imageMap": "rectangle"})).unwrap();
        let original = app.session.active().unwrap().doc.clone();
        crate::io::export(&mut app, Some("jpg"), Some(path("map.jpg")), &json!({"imageMap": "client", "useArtboards": false})).unwrap();
        assert_eq!(shown.requests(), [PickRequest::Folder]);
        assert!(writes.borrow().is_empty());
        grant(&shown, &grants, &path(""));
        poll(&mut app);
        if crate::macos_mas::enabled() {
            assert!(writes.borrow().is_empty(), "the main image also waits for actual native authority");
            assert!(app.ui.status.contains(fileio::FOLDER_REQUIRED), "{}", app.ui.status);
            let st = app.session.active().unwrap();
            assert!(st.path.is_none());
            assert!(Arc::ptr_eq(&original, &st.doc));
        } else {
            assert_eq!(names(&writes), [path("map.jpg"), path("map.html")]);
            assert!(writes.borrow()[1].1.windows(b"https://example.com".len()).any(|w| w == b"https://example.com"));
        }
        assert!(app.session.active().unwrap().is_dirty(), "an export never marks its source saved");
    }

    #[test]
    fn closing_the_original_document_drops_the_prepared_outputs() {
        let (mut app, shown, writes, grants, _) = fixture();
        save_separate(&mut app, &shown, &path("closed.vectorcraft"));
        app.session.execute("file.close", &json!({"index": 0})).unwrap();
        app.sync_views();
        grant(&shown, &grants, &path(""));
        poll(&mut app);
        assert!(writes.borrow().is_empty());
        assert!(app.session.documents().is_empty());
        assert!(app.ui.status.contains("document was closed"), "{}", app.ui.status);
    }

    #[test]
    fn folder_resume_reuses_one_encoded_snapshot_and_preserves_later_edits() {
        let (mut app, shown, writes, grants, history) = fixture();
        let uid = app.session.active().unwrap().uid;
        let plan = fileio::save_plan(
            &app.session,
            fileio::SaveMode::SaveAs,
            &json!({"path": path("once.vectorcraft"), "options": {"separateArtboards": true}}),
        )
        .unwrap();
        let job = fileio::deferred_save_job(&mut app.session, plan).unwrap();
        let done = job.clone();
        let encodes = Arc::new(AtomicUsize::new(0));
        let encodes_copy = encodes.clone();
        let policy = app.output_policy.clone().unwrap();
        run(
            &mut app,
            false,
            "Saving".into(),
            Some(uid),
            policy,
            Box::new(move || {
                encodes_copy.fetch_add(1, Ordering::SeqCst);
                let encoded = job.encode().map_err(|e| e.to_string())?;
                Ok(Prepared::Save { job, encoded })
            }),
            Box::new(move |app, r| {
                done.complete(&mut app.session, uid);
                Ok(r)
            }),
        )
        .unwrap();
        assert_eq!(encodes.load(Ordering::SeqCst), 1);
        assert!(writes.borrow().is_empty());
        app.session.execute("shape.rectangle", &json!({"x": 70, "y": 10, "width": 20, "height": 20})).unwrap();
        let edited = app.session.active().unwrap().doc.clone();
        grant(&shown, &grants, &path(""));
        poll(&mut app);
        assert_eq!(encodes.load(Ordering::SeqCst), 1);
        if crate::macos_mas::enabled() {
            assert!(writes.borrow().is_empty(), "resuming retained bytes still checks actual native authority");
            assert!(app.ui.status.contains(fileio::FOLDER_REQUIRED), "{}", app.ui.status);
            assert!(app.session.active().unwrap().path.is_none());
        } else {
            assert_eq!(writes.borrow().len(), 3);
        }
        let st = app.session.active().unwrap();
        assert!(Arc::ptr_eq(&edited, &st.doc));
        assert!(st.is_dirty(), "edits made while consent was pending are not marked saved");
        assert!(matches!(history.borrow().as_slice(), [PickRequest::Folder]));
    }

    #[test]
    fn ordinary_hosts_keep_multi_output_export_without_a_folder_picker() {
        let (mut app, shown, writes, _, history) = fixture();
        app.output_policy = None;
        let original = app.session.active().unwrap().doc.clone();
        let result = crate::io::export(&mut app, Some("svg"), Some(path("ordinary.svg")), &json!({"useArtboards": true, "range": "all"}));
        if crate::macos_mas::enabled() {
            assert!(result.unwrap_err().contains(fileio::FOLDER_REQUIRED));
            assert!(writes.borrow().is_empty(), "a missing UI policy cannot disable the native writer boundary");
            let st = app.session.active().unwrap();
            assert!(st.path.is_none() && st.is_dirty());
            assert!(Arc::ptr_eq(&original, &st.doc));
        } else {
            result.unwrap();
            assert_eq!(names(&writes), [path("ordinary-Artboard-1.svg"), path("ordinary-Artboard-2.svg")]);
        }
        assert!(shown.requests().is_empty() && history.borrow().is_empty());
    }
}
