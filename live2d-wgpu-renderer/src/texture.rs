use std::path::Path;

use anyhow::{Context, Result};
use image::GenericImageView;
use wgpu::util::DeviceExt;

/// Load a PNG texture from disk and upload it to the GPU.
pub fn load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: &Path,
) -> Result<wgpu::Texture> {
    let img = image::open(path)
        .with_context(|| format!("cannot open texture: {}", path.display()))?;
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();

    let texture = device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: path.file_name().and_then(|s| s.to_str()),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        &rgba,
    );

    Ok(texture)
}
