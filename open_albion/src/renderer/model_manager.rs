// use thunderdome::{Arena, Index as ArenaId};

// use wgpu::util::DeviceExt;

// use crate::RendererBase;

// pub struct ModelManager {
//     buffers: Arena<wgpu::Buffer>,
//     textures: Arena<wgpu::Texture>,
//     meshes: Arena<Mesh>,
//     materials: Arena<Material>,
//     models: Arena<Model>,
// }

// pub struct BufferId (ArenaId);

// pub struct TextureId (ArenaId);

// pub struct MeshId (ArenaId);

// pub struct MaterialId (MaterialId);

// pub struct ModelId (ArenaId);

// pub struct Mesh {
//     vertex_buffer: BufferId,
//     index_buffer: BufferId,
//     count: u32,
// }

// pub struct Material {
//     base_color: TextureId,
//     bind_group: wgpu::BindGroup,
// }

// pub struct Model {
//     material: MaterialId,
//     meshes: Vec<MeshId>,
// }

// impl Resources {
//     pub fn create() -> Self {
//         let buffers = Arena::new();
//         let textures = Arena::new();
//         let meshes = Arena::new();
//         let materials = Arena::new();
//         let models = Arena::new();
//         Self { buffers, textures, meshes, materials, models }
//     }
// }

// impl RendererBase {
//     pub fn create_mesh_with_data<T: AsRef<[u8]>>(
//         &mut self,
//         vertex_data: T,
//         index_format: wgpu::IndexFormat,
//         index_data: T,
//         count: u32,
//     ) -> MeshId {
//         let vertex_data = vertex_data.as_ref();

//         let vertex_buffer = self.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//             label: None,
//             contents: vertex_data,
//             usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
//         });

//         let index_data = index_data.as_ref();

//         let index_buffer = self.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
//             label: None,
//             contents: index_data,
//             usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
//         });

//         let id = self.resources.meshes.insert(Mesh { vertex_buffer, index_buffer, count });

//         MeshId(id)
//     }

//     // pub fn create_buffer(&mut self,)

//     pub fn create_buffer_with_data(&mut self, desc: &wgpu::util::BufferInitDescriptor) -> BufferId {
//         let buffer = self.device.create_buffer_init(desc);
//         let id = self.resources.buffers.insert(buffer);
//         BufferId(id)
//     }
// }