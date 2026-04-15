// Clipping mask renderer for Live2D drawables.
//
// Live2D "clip masks" work by rendering a set of mask drawables into an
// off-screen R8 texture, then sampling that texture in the fragment shader
// of every clipped drawable to discard pixels outside the clip region.
//
// Implementation plan:
//   1. Build a `MaskRenderer` with a fixed-size mask texture (e.g. 256×256)
//      and a render pipeline that writes texture alpha → red channel.
//   2. Before rendering a clip group, run a mask render pass.
//   3. Pass the resulting mask texture to the main pipeline via group(3).
//   4. Update live2d.wgsl to sample the mask and multiply alpha.
//
// TODO: implement this module.  Until then, masked drawables are rendered
// without clipping (visible but not clipped to their mask shape).

pub struct MaskRenderer {
    _mask_texture: wgpu::Texture,
}

impl MaskRenderer {
    pub fn new(_device: &wgpu::Device, _size: (u32, u32)) -> Self {
        todo!("MaskRenderer: implement clip-mask off-screen render pass")
    }
}
