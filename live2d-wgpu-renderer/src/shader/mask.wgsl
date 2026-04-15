// Mask source shader.
// Renders clip-mask drawables into a 256×256 RGBA8 off-screen texture.
// Only the alpha channel of the drawable's texture is written to the R channel
// of the mask texture.  The main shader samples .r from this texture.
//
// group(0) binding(0) : MVP (same matrix as main render)
// group(1) binding(0) : drawable diffuse texture
// group(1) binding(1) : drawable diffuse sampler

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv:       vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       uv:            vec2<f32>,
};

@group(0) @binding(0) var<uniform> u_mvp:    mat4x4<f32>;
@group(1) @binding(0) var          t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var          s_diffuse: sampler;

@vertex
fn vs_mask(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = u_mvp * vec4<f32>(in.position, 0.0, 1.0);
    out.uv = in.uv; // UV V-flip already applied in vertex buffer
    return out;
}

@fragment
fn fs_mask(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(t_diffuse, s_diffuse, in.uv).a;
    // Write alpha to R channel; other channels are unused but set for clarity.
    return vec4<f32>(alpha, alpha, alpha, alpha);
}
