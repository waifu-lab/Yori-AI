use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use live2d_cubism_core_sys::core as live2d;
use serde::Deserialize;

use crate::drawable::BlendMode;

#[derive(Deserialize)]
struct Model3Json {
    #[serde(rename = "FileReferences")]
    file_references: FileReferences,
}

#[derive(Deserialize)]
struct FileReferences {
    #[serde(rename = "Moc")]
    moc: String,
    #[serde(rename = "Textures")]
    textures: Vec<String>,
    #[serde(rename = "Expressions", default)]
    expressions: Vec<ExpressionRef>,
}

#[derive(Deserialize)]
struct ExpressionRef {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "File")]
    file: String,
}

#[derive(Deserialize)]
struct Exp3Json {
    #[serde(rename = "Parameters", default)]
    parameters: Vec<Exp3Param>,
}

#[derive(Deserialize)]
struct Exp3Param {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Value")]
    value: f32,
}

pub struct Expression {
    pub name: String,
    params: Vec<(String, f32)>,
}

impl Expression {
    /// Load all expressions for `dir`.
    ///
    /// If `model3.json` lists expressions under `FileReferences.Expressions`,
    /// those are used (preserving the model-defined order and names).
    /// Otherwise, every `*.exp3.json` file in `dir` is loaded directly.
    pub fn load_all(dir: &Path, model3_json: &str) -> Vec<Self> {
        if let Ok(config) = serde_json::from_str::<Model3Json>(model3_json) {
            if !config.file_references.expressions.is_empty() {
                return config
                    .file_references
                    .expressions
                    .iter()
                    .filter_map(|eref| Self::load_file(dir.join(&eref.file), &eref.name))
                    .collect();
            }
        }

        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| {
                e.path().extension().and_then(|s| s.to_str()) == Some("json")
                    && e.path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.ends_with("exp3"))
                        .unwrap_or(false)
            })
            .collect();
        entries.sort_by_key(|e| e.path());

        entries
            .into_iter()
            .filter_map(|e| {
                let path = e.path();
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    // strip trailing ".exp3" to get a friendly name
                    .map(|s| s.trim_end_matches(".exp3").to_owned())
                    .unwrap_or_default();
                Self::load_file(path, &name)
            })
            .collect()
    }

    fn load_file(path: impl AsRef<Path>, name: &str) -> Option<Self> {
        let text = std::fs::read_to_string(path).ok()?;
        let exp: Exp3Json = serde_json::from_str(&text).ok()?;
        Some(Expression {
            name: name.to_owned(),
            params: exp
                .parameters
                .into_iter()
                .map(|p| (p.id, p.value))
                .collect(),
        })
    }
}

pub struct DrawableSnapshot {
    pub vertex_positions: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub render_order: i32,
    pub opacity: f32,
    pub texture_index: usize,
    pub blend_mode: BlendMode,
    pub is_visible: bool,
    /// Indices of drawables that clip-mask this one.  Empty = unclipped.
    pub masks: Vec<usize>,
    pub is_vertex_dirty: bool,
    /// When true, show the region OUTSIDE the mask instead of inside.
    pub is_inverted_mask: bool,
}

pub struct Live2dModel {
    _core: live2d::CubismCore,
    _moc: live2d::Moc,
    model: live2d::Model,
    pub texture_paths: Vec<PathBuf>,
    dir: PathBuf,
    pub model3_json: String,
}

impl Live2dModel {
    pub fn load(dir: &Path) -> Result<Self> {
        let model3_path = find_model3_json(dir)?;
        let json_str = std::fs::read_to_string(&model3_path)?;
        let config: Model3Json = serde_json::from_str(&json_str)?;

        let moc_path = dir.join(&config.file_references.moc);
        let moc_bytes = std::fs::read(&moc_path)
            .with_context(|| format!("cannot read moc3: {}", moc_path.display()))?;

        let core = live2d::CubismCore::default();
        let moc = core
            .moc_from_bytes(&moc_bytes)
            .context("failed to load moc3")?;
        let model = live2d::Model::from_moc(&moc);

        let texture_paths = config
            .file_references
            .textures
            .iter()
            .map(|p| dir.join(p))
            .collect();

        Ok(Self {
            _core: core,
            _moc: moc,
            model,
            texture_paths,
            dir: dir.to_owned(),
            model3_json: json_str,
        })
    }

    pub fn load_expressions(&self) -> Vec<Expression> {
        Expression::load_all(&self.dir, &self.model3_json)
    }

    pub fn apply_expression(&self, expr: &Expression) {
        for (id, value) in &expr.params {
            self.set_parameter(id, *value);
        }
    }

    pub fn clear_expression(&self) {
        for name in &[
            "ParamExpression_1",
            "ParamExpression_2",
            "ParamExpression_3",
            "ParamExpression_4",
            "ParamHide_EyesL1",
            "ParamHighLightHide_EyesL1",
            "ParamHide_EyeSocket",
            "ParamHide_EyeSocket2",
        ] {
            self.set_parameter(name, 0.0);
        }
    }

    pub fn update(&self) {
        self.model.write_dynamic().update();
    }

    pub fn drawable_count(&self) -> usize {
        self.model.get_static().drawables().len()
    }

    pub fn canvas_size_units(&self) -> (f32, f32) {
        let info = self.model.get_static().canvas_info();
        let (w_px, h_px) = info.size_in_pixels;
        let ppu = info.pixels_per_unit;
        (w_px / ppu, h_px / ppu)
    }

    pub fn drawable_snapshots(&self) -> Vec<DrawableSnapshot> {
        let model_static = self.model.get_static();
        let dynamic = self.model.read_dynamic();

        let drawables = model_static.drawables();
        let render_orders = dynamic.drawable_render_orders();
        let opacities = dynamic.drawable_opacities();
        let vertex_containers = dynamic.drawable_vertex_position_containers();
        let flagsets = dynamic.drawable_dynamic_flagsets();

        drawables
            .iter()
            .enumerate()
            .map(|(i, d)| {
                let flags = flagsets[i];
                let is_visible = flags.contains(live2d::DynamicDrawableFlags::IsVisible);
                let is_vertex_dirty =
                    flags.contains(live2d::DynamicDrawableFlags::VertexPositionsDidChange);

                let vertex_positions = vertex_containers[i].iter().map(|v| [v.x, v.y]).collect();

                let uvs = d.vertex_uvs().iter().map(|v| [v.x, v.y]).collect();

                let cf = d.constant_flagset();
                let blend_mode = if cf.contains(live2d::ConstantDrawableFlags::BlendAdditive) {
                    BlendMode::Additive
                } else if cf.contains(live2d::ConstantDrawableFlags::BlendMultiplicative) {
                    BlendMode::Multiplicative
                } else {
                    BlendMode::Normal
                };

                DrawableSnapshot {
                    vertex_positions,
                    uvs,
                    indices: d.triangle_indices().to_vec(),
                    render_order: render_orders[i],
                    opacity: opacities[i],
                    texture_index: d.texture_index().as_usize(),
                    blend_mode,
                    is_visible,
                    masks: d.masks().to_vec(),
                    is_vertex_dirty,
                    is_inverted_mask: cf.contains(live2d::ConstantDrawableFlags::IsInvertedMask),
                }
            })
            .collect()
    }

    /// Set a named parameter value (e.g. `"ParamMouthOpenY"`, `"ParamEyeLOpen"`).
    pub fn set_parameter(&self, name: &str, value: f32) {
        let params = self.model.get_static().parameters();
        if let Some(idx) = params.iter().position(|p| p.id() == name) {
            let mut dynamic = self.model.write_dynamic();
            if let Some(v) = dynamic.parameter_values_mut().get_mut(idx) {
                *v = value;
            }
        }
    }
}

fn find_model3_json(dir: &Path) -> Result<PathBuf> {
    std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.extension().and_then(|s| s.to_str()) == Some("json")
                && p.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.ends_with("model3"))
                    .unwrap_or(false)
        })
        .context("no .model3.json found in model directory")
}
