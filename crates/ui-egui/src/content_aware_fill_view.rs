//! Content-Aware Fill Workspace (Photoshop 2026 parity):
//! Dedicated taskspace with green sampling brush, sampling area options,
//! rotation adaptation, scale, mirror, and output destination.

use serde_json::{Map, Value, json};

use crate::PhotocraftApp;
use crate::state::DialogKind;
use crate::theme::Tokens;
use crate::widgets;

pub const COMMAND: &str = "edit.contentAwareFill";

/// Open the Content-Aware Fill Workspace dialog.
pub fn open(app: &mut PhotocraftApp) -> u64 {
    let label = photocraft_engine::commands::find(COMMAND).map_or(tl!("Content-Aware Fill…"), |c| c.label);
    let mut f = Map::new();
    f.insert("__contentAwareFill".into(), json!(true));
    f.insert("__command".into(), json!(COMMAND));
    f.insert("__label".into(), json!(label));
    f.insert("__preview".into(), json!(true));
    f.insert("sampling".into(), json!("auto"));
    f.insert("colorAdaptation".into(), json!("default"));
    f.insert("rotationAdaptation".into(), json!("none"));
    f.insert("scale".into(), json!(false));
    f.insert("mirror".into(), json!(false));
    f.insert("output".into(), json!("current"));
    f.insert("sampleAllLayers".into(), json!(false));
    f.insert("__workspaceTool".into(), json!("samplingBrush"));
    f.insert("__brushMode".into(), json!("add"));
    f.insert("__brushSize".into(), json!(50.0));
    f.insert("__showSampling".into(), json!(true));
    f.insert("__samplingOpacity".into(), json!(50.0));
    app.ui.open_dialog(DialogKind::Command, f)
}

/// Whether the dialog fields belong to the Content-Aware Fill Workspace.
pub fn owns(f: &Map<String, Value>) -> bool {
    f.contains_key("__contentAwareFill")
}

fn s<'a>(f: &'a Map<String, Value>, key: &str, default: &'a str) -> &'a str {
    f.get(key).and_then(Value::as_str).unwrap_or(default)
}

fn b(f: &Map<String, Value>, key: &str, default: bool) -> bool {
    f.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn num(f: &Map<String, Value>, key: &str, default: f32) -> f32 {
    f.get(key).and_then(Value::as_f64).map_or(default, |v| v as f32)
}

/// Parameters for `edit.contentAwareFill` constructed from the workspace state.
pub fn params(f: &Map<String, Value>) -> Value {
    let mut p = json!({
        "sampling": s(f, "sampling", "auto"),
        "colorAdaptation": s(f, "colorAdaptation", "default"),
        "rotationAdaptation": s(f, "rotationAdaptation", "none"),
        "scale": b(f, "scale", false),
        "mirror": b(f, "mirror", false),
        "output": s(f, "output", "current"),
        "sampleAllLayers": b(f, "sampleAllLayers", false),
    });
    if let Some(area) = f.get("area") {
        p["area"] = area.clone();
    }
    p
}

/// Confirm (OK): run the command with all workspace parameters.
pub fn confirm(app: &mut PhotocraftApp, f: &Map<String, Value>) -> Result<Value, String> {
    app.run(COMMAND, params(f))
}

/// Workspace settings panel UI for Content-Aware Fill.
pub fn body(_app: &mut PhotocraftApp, ui: &mut egui::Ui, f: &mut Map<String, Value>) {
    let t = Tokens::get(ui.ctx());

    // Workspace Tools Strip
    ui.label(egui::RichText::new("Workspace Tools").strong());
    let mut ws_tool = s(f, "__workspaceTool", "samplingBrush").to_string();
    ui.horizontal_wrapped(|ui| {
        let tools = [
            ("samplingBrush", "Sampling Brush (B)"),
            ("lasso", "Lasso (L)"),
            ("hand", "Hand (H)"),
            ("zoom", "Zoom (Z)"),
        ];
        for (id, label) in tools {
            if widgets::pill_tab(ui, label, ws_tool == id).clicked() {
                ws_tool = id.to_string();
            }
        }
    });
    f.insert("__workspaceTool".into(), json!(ws_tool));

    ui.add_space(4.0);
    // Sampling Brush Options
    let mut subtract = s(f, "__brushMode", "add") == "subtract";
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Brush Mode").color(t.text_dim));
        let options = [(false, "+ Add to Sampling Area"), (true, "- Exclude from Sampling Area")];
        widgets::dropdown(ui, "caf-brush-mode", &mut subtract, &options, 180.0);
    });
    f.insert("__brushMode".into(), json!(if subtract { "subtract" } else { "add" }));

    let mut brush_size = num(f, "__brushSize", 50.0);
    widgets::slider_row(ui, "Size", &mut brush_size, 1.0..=500.0, "px", None);
    f.insert("__brushSize".into(), json!(brush_size.round().clamp(1.0, 500.0)));

    ui.add_space(4.0);
    widgets::hairline(ui);
    ui.add_space(4.0);

    // Sampling Area Overlay Options
    ui.label(egui::RichText::new("Sampling Area Overlay").strong());
    let mut show_overlay = b(f, "__showSampling", true);
    widgets::checkbox(ui, &mut show_overlay, "Show Sampling Area");
    f.insert("__showSampling".into(), json!(show_overlay));

    if show_overlay {
        let mut opacity = num(f, "__samplingOpacity", 50.0);
        widgets::slider_row(ui, "Overlay Opacity", &mut opacity, 0.0..=100.0, "%", None);
        f.insert("__samplingOpacity".into(), json!(opacity.round().clamp(0.0, 100.0)));
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Overlay Color").color(t.text_dim));
            ui.label(egui::RichText::new("● Green (#00E600)").color(egui::Color32::from_rgb(0, 230, 0)));
        });
    }

    ui.add_space(4.0);
    widgets::hairline(ui);
    ui.add_space(4.0);

    // Sampling Area Options
    ui.label(egui::RichText::new("Sampling Area Options").strong());
    let sampling_opts = [
        ("auto".to_string(), "Auto"),
        ("rectangular".to_string(), "Rectangular"),
        ("custom".to_string(), "Custom"),
    ];
    let mut sampling = s(f, "sampling", "auto").to_string();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Sampling").color(t.text_dim));
        widgets::dropdown(ui, "caf-sampling", &mut sampling, &sampling_opts, 140.0);
    });
    f.insert("sampling".into(), json!(sampling));

    let mut sample_all = b(f, "sampleAllLayers", false);
    widgets::checkbox(ui, &mut sample_all, "Sample All Layers");
    f.insert("sampleAllLayers".into(), json!(sample_all));

    ui.add_space(4.0);
    widgets::hairline(ui);
    ui.add_space(4.0);

    // Fill Settings
    ui.label(egui::RichText::new("Fill Settings").strong());
    let color_opts = [
        ("none".to_string(), "None"),
        ("default".to_string(), "Default"),
        ("medium".to_string(), "Medium"),
        ("high".to_string(), "High"),
        ("veryHigh".to_string(), "Very High"),
    ];
    let mut color_adapt = s(f, "colorAdaptation", "default").to_string();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Color Adaptation").color(t.text_dim));
        widgets::dropdown(ui, "caf-color-adaptation", &mut color_adapt, &color_opts, 140.0);
    });
    f.insert("colorAdaptation".into(), json!(color_adapt));

    let rot_opts = [
        ("none".to_string(), "None"),
        ("low".to_string(), "Low"),
        ("medium".to_string(), "Medium"),
        ("full".to_string(), "Full"),
    ];
    let mut rot_adapt = s(f, "rotationAdaptation", "none").to_string();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Rotation Adaptation").color(t.text_dim));
        widgets::dropdown(ui, "caf-rot-adaptation", &mut rot_adapt, &rot_opts, 140.0);
    });
    f.insert("rotationAdaptation".into(), json!(rot_adapt));

    let mut scale = b(f, "scale", false);
    widgets::checkbox(ui, &mut scale, "Scale");
    f.insert("scale".into(), json!(scale));

    let mut mirror = b(f, "mirror", false);
    widgets::checkbox(ui, &mut mirror, "Mirror");
    f.insert("mirror".into(), json!(mirror));

    ui.add_space(4.0);
    widgets::hairline(ui);
    ui.add_space(4.0);

    // Output Settings
    ui.label(egui::RichText::new("Output Settings").strong());
    let output_opts = [
        ("current".to_string(), "Current Layer"),
        ("new".to_string(), "New Layer"),
        ("duplicate".to_string(), "Duplicate Layer"),
    ];
    let mut output = s(f, "output", "current").to_string();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Output To").color(t.text_dim));
        widgets::dropdown(ui, "caf-output", &mut output, &output_opts, 140.0);
    });
    f.insert("output".into(), json!(output));

    ui.add_space(6.0);
    if widgets::secondary_button(ui, "Reset to Defaults", 140.0).clicked() {
        f.insert("sampling".into(), json!("auto"));
        f.insert("colorAdaptation".into(), json!("default"));
        f.insert("rotationAdaptation".into(), json!("none"));
        f.insert("scale".into(), json!(false));
        f.insert("mirror".into(), json!(false));
        f.insert("output".into(), json!("current"));
        f.insert("sampleAllLayers".into(), json!(false));
        f.insert("__brushMode".into(), json!("add"));
        f.insert("__brushSize".into(), json!(50.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_params_serialization() {
        let mut f = Map::new();
        f.insert("__contentAwareFill".into(), json!(true));
        assert!(owns(&f));

        f.insert("sampling".into(), json!("custom"));
        f.insert("colorAdaptation".into(), json!("veryHigh"));
        f.insert("rotationAdaptation".into(), json!("full"));
        f.insert("scale".into(), json!(true));
        f.insert("mirror".into(), json!(true));
        f.insert("output".into(), json!("new"));
        f.insert("sampleAllLayers".into(), json!(true));

        let p = params(&f);
        assert_eq!(p["sampling"], "custom");
        assert_eq!(p["colorAdaptation"], "veryHigh");
        assert_eq!(p["rotationAdaptation"], "full");
        assert_eq!(p["scale"], true);
        assert_eq!(p["mirror"], true);
        assert_eq!(p["output"], "new");
        assert_eq!(p["sampleAllLayers"], true);
    }

    #[test]
    fn open_dialog_and_menu_invocation() {
        let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
        app.run("file.new", json!({"width": 40, "height": 30, "background": "transparent"})).unwrap();
        app.run("select.all", json!({})).unwrap();
        let en = (photocraft_engine::commands::find("edit.contentAwareFill").unwrap().enabled)(&app.session);
        assert!(en.is_ok(), "{en:?}");
        let r = crate::menus::invoke_unguarded(&mut app, &egui::Context::default(), "edit.contentAwareFill", json!({})).unwrap();
        println!("R = {r:?}");
        println!("DIALOGS = {:?}", app.ui.dialogs.iter().map(|d| &d.fields).collect::<Vec<_>>());
        assert!(r.get("dialog").is_some(), "{r:?}");
        assert!(app.ui.dialogs.iter().any(|d| owns(&d.fields)));
    }
}
