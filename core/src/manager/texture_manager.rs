use std::{collections::HashMap, ops::Not};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Device, Extent3d, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, TexelCopyTextureInfo, Texture,
    TextureFormat, TextureUsages, TextureViewDescriptor,
};

use crate::{manager::asset_manager::AssetManager, renderer::Renderer};

pub type TextureManagerError = String;

struct TextureObject {
    texture: Texture,
    bind_group: BindGroup,
}

pub struct TextureManager<'r> {
    renderer: &'r Renderer,
    textures_map: HashMap<String, TextureObject>,
    texture_sampler: Sampler,
    texture_bind_group_layout: BindGroupLayout,
}

impl<'r> TextureManager<'r> {
    pub fn new(renderer: &'r Renderer) -> Self {
        let (device, _) = renderer.borrow_device();
        Self {
            renderer,
            textures_map: HashMap::new(),
            texture_sampler: Self::create_sampler(device),
            texture_bind_group_layout: Self::create_bind_group_layout(device),
        }
    }

    // TODO
    // 3. unloader
    pub fn load_texture(
        &mut self,
        asset_mgr: &AssetManager,
        texture_id: &str,
    ) -> Result<(), TextureManagerError> {
        let texture_id = texture_id.to_owned();
        if self.textures_map.contains_key(&texture_id) {
            return Err(format!("texture {} is loaded", texture_id));
        }

        let (device, queue) = self.renderer.borrow_device();
        let bytes = asset_mgr.load_bytes(&texture_id)?;

        // create gpu texture buffer
        let texture_img = image::load_from_memory(bytes.as_slice()).unwrap();
        let texture_img_rgba = texture_img.to_rgba8();
        let texture_size = Extent3d {
            width: texture_img_rgba.dimensions().0,
            height: texture_img_rgba.dimensions().1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(format!("{}_texture", texture_id).as_ref()),
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            size: texture_size,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[], //TextureFormat::Rgba8UnormSrgb]
        });
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &texture_img_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * texture_size.width),
                rows_per_image: Some(texture_size.height),
            },
            texture_size,
        );

        // create bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{}_texture_bind_group", texture_id).as_ref()),
            layout: &self.texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &texture.create_view(&TextureViewDescriptor::default()),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
        });

        let texture_object = TextureObject {
            texture: texture,
            bind_group: texture_bind_group,
        };
        self.textures_map.insert(texture_id, texture_object);

        Ok(())
    }

    pub fn unload(&mut self, texture_id: &str) -> Result<(), TextureManagerError> {
        if self.textures_map.contains_key(texture_id).not() {
            return Err(format!("texture {} is not exists", texture_id));
        }

        let texture_obj = self.textures_map.get(texture_id).unwrap();
        texture_obj.texture.destroy();
        self.textures_map.remove(texture_id);

        Ok(())
    }

    pub(crate) fn borrow_bind_group_layout(&self) -> &BindGroupLayout {
        &self.texture_bind_group_layout
    }

    fn create_sampler(device: &Device) -> Sampler {
        let texture_sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        texture_sampler
    }

    fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        texture_bind_group_layout
    }
}
