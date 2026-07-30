use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bytemuck::Pod;
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer, BufferUsages,
    ShaderStages,
};

use crate::{renderer::SharedRenderer, uniform::SizedFn};

pub type SharedUniformBufferManager = Arc<RwLock<UniformBufferManager>>;

struct UniformBufferObject {
    buffers: HashMap<String, Buffer>,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

pub struct UniformBufferEntry {
    pub size_fn: SizedFn,
    pub buffer_name: String,
    pub binding: u32,
    pub items_count: u64,
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

    pub fn create(&mut self, bind_group_id: &str, entries: Vec<UniformBufferEntry>) {
        let renderer = self.renderer.read().unwrap();
        let (device, _) = renderer.borrow_device();

        let binding_entries: Vec<BindGroupLayoutEntry> = entries
            .iter()
            .map(|p| BindGroupLayoutEntry {
                binding: p.binding,
                visibility: ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &binding_entries,
                label: Some(format!("{}_uniform_bind_group_layout", bind_group_id).as_ref()),
            });

        let buffers: Vec<(u32, String, Buffer)> = entries
            .into_iter()
            .map(|p| {
                (
                    p.binding,
                    p.buffer_name,
                    device.create_buffer(&wgpu::wgt::BufferDescriptor {
                        label: None,
                        size: (p.size_fn)() * p.items_count,
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    }),
                )
            })
            .collect();
        let bind_group_entries: Vec<BindGroupEntry> = buffers
            .iter()
            .map(|b| BindGroupEntry {
                binding: b.0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &b.2,
                    size: None,
                    offset: 0,
                }),
            })
            .collect();
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{}_uniform_bind_group", bind_group_id).as_ref()),
            layout: &uniform_bind_group_layout,
            entries: &bind_group_entries,
        });

        let buffers_map: HashMap<String, Buffer> =
            buffers.into_iter().map(|b| (b.1, b.2)).collect();

        let uniform_buffer_object = UniformBufferObject {
            buffers: buffers_map,
            bind_group: uniform_bind_group,
            bind_group_layout: uniform_bind_group_layout,
        };
        self.buffers_map
            .insert(bind_group_id.to_owned(), uniform_buffer_object);
    }

    pub fn write_from_beginning<T>(&self, bind_group_id: &str, buffer_name: &str, data: Vec<T>)
    where
        T: Pod,
    {
        let uniform_buffer_object = self.buffers_map.get(bind_group_id).unwrap();
        let renderer = self.renderer.read().unwrap();
        let (_, queue) = renderer.borrow_device();
        queue.write_buffer(
            &uniform_buffer_object.buffers[buffer_name],
            0,
            &bytemuck::cast_slice(&data),
        );
    }

    pub fn insert<T>(&self, bind_group_id: &str, buffer_name: &str, data: &T, index: u64)
    where
        T: Pod,
    {
        let size = size_of::<T>() as u64;
        let uniform_buffer_object = self.buffers_map.get(bind_group_id).unwrap();
        let renderer = self.renderer.read().unwrap();
        let (_, queue) = renderer.borrow_device();
        queue.write_buffer(
            &uniform_buffer_object.buffers[buffer_name],
            size * index,
            &bytemuck::bytes_of(data),
        );
    }

    pub fn borrow_bind_group(&self, bind_group_id: &str) -> Option<(&BindGroup, &BindGroupLayout)> {
        let uniform_buffer_object = self.buffers_map.get(bind_group_id);
        if uniform_buffer_object.is_none() {
            return None;
        }
        let obj = uniform_buffer_object.unwrap();
        Some((&obj.bind_group, &obj.bind_group_layout))
    }

    pub fn cleanup(&mut self, bind_group_id: &str) {
        let uniform_buffer_object = self.buffers_map.get(bind_group_id).unwrap();
        uniform_buffer_object
            .buffers
            .values()
            .for_each(|b| b.destroy());
        self.buffers_map.remove(bind_group_id);
    }

    pub fn cleanup_all(&mut self) {
        let buffers_names: Vec<String> = self.buffers_map.keys().cloned().collect();

        for buffer_name in buffers_names {
            let uniform_buffer_object = self.buffers_map.get(&buffer_name).unwrap();
            uniform_buffer_object
                .buffers
                .values()
                .for_each(|b| b.destroy());
            self.buffers_map.remove(&buffer_name);
        }
    }
}
