use anyhow::*;
use bevy_ecs::prelude::{Component, Bundle};
use ne_app::types::Name;

/// A material stored as bundle in bevy ecs.
#[derive(Bundle)]
pub struct NamedMaterial {
    pub name:Name,
    pub material:Material,
}
#[derive(Component)]
pub struct Material {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
/// engine default material.
/// yeah don't know how to implement this here...
// impl Default for Material {
//     fn default() -> Self {
//         Self { texture: Default::default(), view: Default::default(), sampler: Default::default() }
//     }
// }
//For now only a simple texture.
///example:
///``let bytes = include_bytes!("grid.png");``
///``let label = Some("grid.png");``
///  let mat = Material::from_bytes(&device, &queue, bytes, label);
impl Material {
    /*     pub fn from_descriptor(
           device: &wgpu::Device,
           queue: &wgpu::Queue,
           mat_descriptor: &MaterialDescriptor,
       ) -> Result<Self> {
           Self::from_bytes(
               device, queue,
               mat_descriptor.bytes, mat_descriptor.label
           )
       }
    */
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: Option<&str>,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes).unwrap();
        let rgba = img.to_rgba8();
        let dimensions = image::GenericImageView::dimensions(&img);

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            //TODO accessibility
            //ClampToEdge: Any texture coordinates outside the texture will return the color of the nearest pixel on the edges of the texture.
            //Repeat: The texture will repeat as texture coordinates exceed the texture's dimensions.
            //MirrorRepeat: Similar to Repeat, but the image will flip when going over boundaries.
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
