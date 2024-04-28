use image::{GenericImageView, ImageError};

pub fn tex_from_bytes(
    bytes: &[u8],
    device: &wgpu::Device,
    queue: &wgpu::Queue
) -> Result<(wgpu::Texture, wgpu::TextureView), ImageError> {

    let image = image::load_from_memory(bytes)?;
    let dim = image.dimensions();
    let image_rgba = image.to_rgba8();

    let size = wgpu::Extent3d {
        width: dim.0,
        height: dim.1,
        depth_or_array_layers: 1
    };

    let tex = device.create_texture(
        &wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[]
        }
    );

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All
        },
        &image_rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dim.0),
            rows_per_image: Some(dim.1)
        },
        wgpu::Extent3d {
            width: dim.0,
            height: dim.1,
            depth_or_array_layers: 1
        },
    );

    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

    Ok((tex, view))
}
