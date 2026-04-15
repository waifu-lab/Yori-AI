// ── Bind groups ─────────────────────────────────────────────────────────────
// group(0) binding(0) : MVP matrix  (shared per frame)
// group(1) binding(0) : diffuse texture
// group(1) binding(1) : diffuse sampler
// group(2) binding(0) : per-drawable params — vec4(opacity, is_inverted_mask, 0, 0)
// group(3) binding(0) : mask texture      ← only in masked pipelines
// group(3) binding(1) : mask sampler      ← only in masked pipelines
// ────────────────────────────────────────────────────────────────────────────

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv:       vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       uv:            vec2<f32>,
    @location(1)       ndc_pos:       vec2<f32>, // pre-divide NDC for mask UV
};

@group(0) @binding(0) var<uniform> u_mvp:    mat4x4<f32>;
@group(1) @binding(0) var          t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var          s_diffuse: sampler;
@group(2) @binding(0) var<uniform> u_params:  vec4<f32>; // x=opacity, y=is_inverted_mask

// ── shared vertex shader ────────────────────────────────────────────────────

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let pos = u_mvp * vec4<f32>(in.position, 0.0, 1.0);
    out.clip_position = pos;
    out.uv = in.uv;        // UV V-flip already applied in Rust vertex buffer
    out.ndc_pos = pos.xy;  // orthographic → w == 1, no divide needed
    return out;
}

// ── unmasked fragment ───────────────────────────────────────────────────────

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.uv);
    return color * u_params.x;
}

// ── masked fragment (group 3 required) ─────────────────────────────────────

@group(3) @binding(0) var t_mask: texture_2d<f32>;
@group(3) @binding(1) var s_mask: sampler;

@fragment
fn fs_masked(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.uv);

    // NDC [-1,1] → mask texture UV [0,1].
    // NDC y=+1 is top → texture v=0 is top, so flip y.
    let mask_uv = vec2<f32>(
        (in.ndc_pos.x + 1.0) * 0.5,
        (1.0 - in.ndc_pos.y) * 0.5,
    );
    var mask_alpha = textureSample(t_mask, s_mask, mask_uv).r;

    // IsInvertedMask: show the region OUTSIDE the mask shape.
    mask_alpha = mix(mask_alpha, 1.0 - mask_alpha, u_params.y);

    return color * (u_params.x * mask_alpha);
}
