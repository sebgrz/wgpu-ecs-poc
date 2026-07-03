use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bytemuck::Pod;
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer, BufferUsages,
    ShaderStages,
};

use crate::renderer::SharedRenderer;

pub type SharedUniformBufferManager = Arc<RwLock<UniformBufferManager>>;

struct UniformBufferObject {
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

pub struct UniformBufferManager {
    renderer: SharedRenderer,
    buffers_map: HashMap<String, UniformBufferObject>,
}

impl<'r> UniformBufferManager {
    pub fn new(renderer: SharedRenderer) -> Self {
        Self {
            renderer,
            buffers_map: HashMap::new(),
        }
    }

    pub fn create<T>(&mut self, buffer_id: &str, items_count: u64) {
        let renderer = self.renderer.read().unwrap();
        let (device, _) = renderer.borrow_device();
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0, // TODO: should be control from function parameter
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some(format!("{}_uniform_bind_group_layout", buffer_id).as_ref()),
            });

        let uniform_buffer = device.create_buffer(&wgpu::wgt::BufferDescriptor {
            label: None,
            size: size_of::<T>() as u64 * items_count,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{}_uniform_bind_group", buffer_id).as_ref()),
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    size: None,
                    offset: 0,
                }),
            }],
        });

        let uniform_buffer_object = UniformBufferObject {
            buffer: uniform_buffer,
            bind_group: uniform_bind_group,
            bind_group_layout: uniform_bind_group_layout,
        };
        self.buffers_map
            .insert(buffer_id.to_owned(), uniform_buffer_object);
    }

    pub fn write_from_beginning<T>(&self, buffer_id: &str, data: Vec<T>)
    where
        T: Pod,
    {
        let uniform_buffer_object = self.buffers_map.get(buffer_id).unwrap();
        let renderer = self.renderer.read().unwrap();
        let (_, queue) = renderer.borrow_device();
        queue.write_buffer(&uniform_buffer_object.buffer, 0, &bytemuck::cast_vec(data));
    }

    pub fn insert<T>(&self, buffer_id: &str, data: &T, index: u64)
    where
        T: Pod,
    {
        let size = size_of::<T>() as u64;
        let uniform_buffer_object = self.buffers_map.get(buffer_id).unwrap();
        let renderer = self.renderer.read().unwrap();
        let (_, queue) = renderer.borrow_device();
        queue.write_buffer(
            &uniform_buffer_object.buffer,
            size * index,
            &bytemuck::bytes_of(data),
        );
    }

    pub fn borrow_bind_group(&self, buffer_id: &str) -> Option<(&BindGroup, &BindGroupLayout)> {
        let uniform_buffer_object = self.buffers_map.get(buffer_id);
        if uniform_buffer_object.is_none() {
            return None;
        }
        let obj = uniform_buffer_object.unwrap();
        Some((&obj.bind_group, &obj.bind_group_layout))
    }

    pub fn cleanup(&mut self, buffer_id: &str) {
        let uniform_buffer_object = self.buffers_map.get(buffer_id).unwrap();
        uniform_buffer_object.buffer.destroy();
        self.buffers_map.remove(buffer_id);
    }
}
