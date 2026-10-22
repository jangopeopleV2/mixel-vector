//! Render the real MIXEL Vector workspace and About UI without a native window.
//!
//! After building the current sources, run the snapshot example with an edited native file:
//! `snapshot --open /absolute/path/edited.vectorcraft --out /absolute/path/ui-evidence`
//! Optional `--size 1440x900 --scale 1` changes the window points and pixel density.
//! Workspace/About PNGs, painted text, control replies and a JSON report are saved.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use egui::epaint::Shape;
use egui_kittest::{Harness, LazyRenderer, TestRenderer};
use serde_json::{Value, json};
use vectorcraft_engine::Session;
use vectorcraft_engine::file_access::{self, AutomationRoots};
use vectorcraft_ui_egui::control::{ControlRequest, Outcome};
use vectorcraft_ui_egui::{Services, VectorcraftApp};

struct Options {
    open: PathBuf,
    out: PathBuf,
    size: egui::Vec2,
    scale: f32,
}

#[derive(Clone)]
struct CanvasUpload {
    width: usize,
    height: usize,
    nontransparent_pixels: usize,
}

type TextureUploads = Rc<RefCell<BTreeMap<egui::TextureId, CanvasUpload>>>;

/// Observe the actual uploads consumed by Harness, then use its normal renderer.
/// Harness drains each frame's texture deltas before exposing `output()`.
struct EvidenceRenderer {
    inner: LazyRenderer,
    uploads: TextureUploads,
}

impl TestRenderer for EvidenceRenderer {
    fn handle_delta(&mut self, delta: &mut egui::TexturesDelta) {
        let mut uploads = self.uploads.borrow_mut();
        for (id, updates) in &delta.set {
            for update in updates {
                if update.pos.is_some() {
                    // A partial update cannot prove the complete canvas raster.
                    uploads.remove(id);
                    continue;
                }
                let egui::ImageData::Color(image) = &update.image;
                uploads.insert(
                    *id,
                    CanvasUpload {
                        width: image.width(),
                        height: image.height(),
                        nontransparent_pixels: image.pixels.iter().filter(|pixel| pixel.a() > 0).count(),
                    },
                );
            }
        }
        for id in &delta.free {
            uploads.remove(id);
        }
        drop(uploads);
        self.inner.handle_delta(delta);
    }

    fn render(&mut self, ctx: &egui::Context, output: &egui::FullOutput) -> Result<image::RgbaImage, String> {
        self.inner.render(ctx, output)
    }
}

fn options() -> Result<Options, String> {
    let mut open = None;
    let mut out = None;
    let mut size = egui::vec2(1440.0, 900.0);
    let mut scale = 1.0;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or_else(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--open" => open = Some(PathBuf::from(value)),
            "--out" => out = Some(PathBuf::from(value)),
            "--size" => {
                let (width, height) = value.split_once('x').ok_or("--size must be WIDTHxHEIGHT")?;
                size = egui::vec2(width.parse().map_err(|_| "invalid width")?, height.parse().map_err(|_| "invalid height")?);
            }
            "--scale" => scale = value.parse::<f32>().map_err(|_| "invalid scale")?,
            other => return Err(format!("unknown option {other}")),
        }
    }
    if !size.x.is_finite() || !size.y.is_finite() || !(800.0..=4096.0).contains(&size.x) || !(600.0..=4096.0).contains(&size.y) {
        return Err("window dimensions must be finite, 800..4096 by 600..4096 points".into());
    }
    if !scale.is_finite() || !(0.5..=2.0).contains(&scale) {
        return Err("--scale must be finite and between 0.5 and 2".into());
    }
    let open = open.ok_or("--open needs the edited native document")?.canonicalize().map_err(|e| e.to_string())?;
    let out = out.ok_or("--out needs an evidence directory")?;
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let out = out.canonicalize().map_err(|e| e.to_string())?;
    Ok(Options { open, out, size, scale })
}

fn control(app: &mut VectorcraftApp, ctx: &egui::Context, method: &str, params: Value) -> Result<Value, String> {
    let (request, _reply) = ControlRequest::new(method, params);
    match vectorcraft_ui_egui::control::handle(app, ctx, &request) {
        Outcome::Done(value) if value.get("ok").and_then(Value::as_bool) == Some(true) => Ok(value),
        Outcome::Done(value) => Err(format!("{method}: {value}")),
        Outcome::Screenshot { .. } => Err(format!("{method} requires a presented frame; use the offscreen renderer")),
    }
}

fn painted_text(output: &egui::FullOutput, viewport: egui::Rect) -> String {
    fn collect(shape: &Shape, clip: egui::Rect, viewport: egui::Rect, text: &mut String) {
        match shape {
            Shape::Text(shape)
                if !shape.galley.elided
                    && clip.contains_rect(shape.visual_bounding_rect())
                    && viewport.contains_rect(shape.visual_bounding_rect()) =>
            {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, clip, viewport, text)),
            _ => {}
        }
    }
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, clipped.clip_rect, viewport, &mut text);
    }
    text
}

fn evidence_path(out: &std::path::Path, name: &str) -> Result<PathBuf, String> {
    let path = out.join(name);
    file_access::check_write(path.to_str().ok_or("evidence path is not UTF-8")?)?;
    Ok(path)
}

fn painted_canvas_meshes(output: &egui::FullOutput, texture: egui::TextureId, canvas: egui::Rect) -> usize {
    fn count(shape: &Shape, texture: egui::TextureId, visible: egui::Rect) -> usize {
        match shape {
            Shape::Mesh(mesh) if mesh.texture_id == texture && mesh.indices.len() >= 3 && mesh.calc_bounds().intersect(visible).is_positive() => 1,
            Shape::Vec(shapes) => shapes.iter().map(|shape| count(shape, texture, visible)).sum(),
            _ => 0,
        }
    }
    output.shapes.iter().map(|clipped| count(&clipped.shape, texture, clipped.clip_rect.intersect(canvas))).sum()
}

fn capture(harness: &mut Harness<'_, VectorcraftApp>, uploads: &TextureUploads, out: &std::path::Path, name: &str) -> Result<Value, String> {
    harness.run_steps(6);
    let app = harness.state();
    if app.ui.status.contains("Internal error:") {
        return Err(format!("{name}: real UI did not render the document canvas: {}", app.ui.status));
    }
    let document = app.session.active().ok_or_else(|| format!("{name}: no active document"))?;
    let key = app.canvas.key.as_ref().ok_or_else(|| format!("{name}: no canvas raster key"))?;
    if key.doc != document.uid as usize || key.revision != document.revision {
        return Err(format!("{name}: canvas raster belongs to a different document or revision"));
    }
    let texture = app.canvas.texture.as_ref().ok_or_else(|| format!("{name}: no canvas texture"))?;
    let upload = uploads.borrow().get(&texture.id()).cloned().ok_or_else(|| format!("{name}: no complete upload of the canvas texture"))?;
    if [upload.width, upload.height] != texture.size() || upload.nontransparent_pixels < 64 {
        return Err(format!("{name}: empty or incomplete artwork upload ({} nontransparent pixels)", upload.nontransparent_pixels));
    }
    let canvas = app.canvas_rect.ok_or_else(|| format!("{name}: no canvas bounds"))?;
    let painted_meshes = painted_canvas_meshes(harness.output(), texture.id(), canvas);
    if painted_meshes == 0 {
        return Err(format!("{name}: current artwork texture was not painted inside the visible canvas"));
    }
    let canvas_evidence = json!({
        "documentUid": document.uid, "revision": key.revision, "textureId": format!("{:?}", texture.id()),
        "width": upload.width, "height": upload.height, "nontransparentPixels": upload.nontransparent_pixels,
        "visiblePaintedMeshes": painted_meshes, "bounds": [canvas.min.x, canvas.min.y, canvas.max.x, canvas.max.y]
    });
    let text = painted_text(harness.output(), harness.ctx.content_rect());
    std::fs::write(evidence_path(out, &format!("{name}-text.txt"))?, &text).map_err(|e| e.to_string())?;
    let image = harness.render()?;
    if image.width() < 400 || image.height() < 300 {
        return Err(format!("{name}: rendered UI dimensions too small"));
    }
    let colors: std::collections::BTreeSet<_> = image.pixels().map(|pixel| pixel.0).collect();
    if colors.len() < 128 {
        return Err(format!("{name}: empty UI snapshot ({} distinct colors)", colors.len()));
    }
    let path = evidence_path(out, &format!("{name}.png"))?;
    image.save(&path).map_err(|e| e.to_string())?;
    Ok(json!({
        "path": path, "width": image.width(), "height": image.height(), "distinctColors": colors.len(), "paintedText": text,
        "canvas": canvas_evidence
    }))
}

fn run(options: &Options) -> Result<Value, String> {
    let read_root = options.open.parent().ok_or("input has no parent directory")?;
    let roots = AutomationRoots::new(Some(read_root), Some(&options.out))?.ok_or("snapshot requires read/write roots")?;
    // Every thread and Harness frame stays confined, including linked-artwork
    // refreshes that run without synthetic control input.
    if !file_access::confine_process(roots.clone()) {
        return Err("snapshot process already has a different filesystem scope".into());
    }
    let input = options.open.to_str().ok_or("input path is not UTF-8")?;
    let mut session = Session::new();
    session.prefs.interface_language = "en".into();
    let opened = session.execute("document.open", &json!({"path": input})).map_err(|e| e.to_string())?;
    if opened.get("warnings").and_then(Value::as_array).is_some_and(|warnings| !warnings.is_empty()) {
        return Err(format!("native document import warnings: {opened}"));
    }
    let document = session.active().ok_or("document.open did not activate a document")?;
    let mut art = 0;
    document.doc.walk(|node| {
        if !matches!(node.kind_label(), "Layer" | "Group" | "Clip Group") {
            art += 1;
        }
    });
    if art == 0 || document.doc.artboards.is_empty() {
        return Err("native document has no art or artboard".into());
    }
    let services = Services {
        read: Some(Box::new(|path| std::fs::read(path).map_err(|e| e.to_string()))),
        write: Some(Box::new(|path, bytes| vectorcraft_format::write_atomic(std::path::Path::new(path), bytes).map_err(|e| e.to_string()))),
        ..Default::default()
    };
    let mut app = VectorcraftApp::new(session, services).with_automation_roots(Some(roots));
    // The same synchronous canvas-rendering path used by existing UI tests: no
    // worker race between the real UI frame and the offscreen snapshot.
    app.canvas.worker_started = true;
    let uploads = TextureUploads::default();
    let renderer = EvidenceRenderer { inner: LazyRenderer::default(), uploads: uploads.clone() };
    let mut harness = Harness::builder().with_size(options.size).with_pixels_per_point(options.scale).renderer(renderer).build_ui_state(
        |ui, app: &mut VectorcraftApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    harness.input_mut().max_texture_side = Some(8192);
    harness.run_steps(6);
    let ctx = harness.ctx.clone();
    let fit = control(harness.state_mut(), &ctx, "engine.execute", json!({"command": "view.fitArtboard", "params": {}}))?;
    let workspace = capture(&mut harness, &uploads, &options.out, "workspace")?;
    let commands = control(harness.state_mut(), &ctx, "engine.commands", json!({}))?;
    let about_command = commands
        .get("result")
        .and_then(Value::as_array)
        .and_then(|commands| commands.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("help.about")));
    if about_command.and_then(|command| command.get("label")).and_then(Value::as_str) != Some("About MIXEL Vector") {
        return Err(format!("incorrect About command name: {about_command:?}"));
    }
    let about_reply = control(harness.state_mut(), &ctx, "engine.execute", json!({"command": "help.about", "params": {}}))?;
    let about = capture(&mut harness, &uploads, &options.out, "about")?;
    let text = about.get("paintedText").and_then(Value::as_str).ok_or("About did not paint text")?;
    for expected in ["About MIXEL Vector", "MIXEL Vector", "MIT OR Apache-2.0.", "Noto Sans, Noto Serif, Inter, JetBrains Mono"] {
        if !text.contains(expected) {
            return Err(format!("About UI is missing {expected:?}"));
        }
    }
    if text.contains("Source Sans") || text.contains("Source Serif") || text.contains("SourceSans") || text.contains("SourceSerif") {
        return Err("About UI contains retired font names".into());
    }
    let state = control(harness.state_mut(), &ctx, "ui.inspect", json!({}))?;
    if state["result"]["ui"]["about"] != true {
        return Err("About UI did not stay open".into());
    }
    let replies = json!({"opened": opened, "fit": fit, "aboutCommand": about_command, "about": about_reply, "uiInspect": state});
    std::fs::write(evidence_path(&options.out, "control-replies.json")?, serde_json::to_vec_pretty(&replies).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    println!("PASS MIXEL Vector offscreen workspace and About: real UI, native artwork, product/font credits, PNG snapshots");
    Ok(json!({"status": "passed", "source": options.open, "artObjects": art, "workspace": workspace, "about": about}))
}

fn main() {
    let result = options().and_then(|options| {
        let result = run(&options);
        let report = match &result {
            Ok(report) => report.clone(),
            Err(error) => json!({"status": "failed", "source": options.open, "error": error}),
        };
        std::fs::write(evidence_path(&options.out, "report.json")?, serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        result.map(|_| ())
    });
    if let Err(error) = result {
        eprintln!("Vector snapshot: {error}");
        std::process::exit(1);
    }
}
