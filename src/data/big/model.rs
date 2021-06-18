use std::mem;
use std::collections::HashMap;

use crate::{Bytes,BigMeshInfo,F16,Vector3Packed};

use mint::{ColumnMatrix3x4,ColumnMatrix4,Quaternion,Vector2,Vector3,Vector4};

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub animated: bool,
    pub bounding_sphere: BoundingSphere,
    pub bounding_box: BoundingBox,
    pub helper_points: Vec<HelperPoint>,
    pub helper_dummies: Vec<HelperDummy>,
    pub helper_point_names: Vec<u8>,
    pub helper_dummy_names: Vec<u8>,
    pub material_count: u32,
    pub primitive_count: u32,
    pub bone_names: BoneNames,
    pub bones: Vec<Bone>,
    pub bone_keyframes: Vec<Keyframe>,
    pub bone_transforms: Vec<ColumnMatrix4<f32>>,
    pub cloth: u8,
    pub unknown4: u16,
    pub unknown5: u16,
    pub transform_matrix: ColumnMatrix3x4<f32>,
    pub materials: Vec<Material>,
    pub primitives: Vec<Primitive>,
}

#[derive(Debug)]
pub struct HelperPoint {
    pub bank_id: u32, // maybe CRC?
    pub point: Vector3<f32>,
    pub bone_index: i32,
}

#[derive(Debug)]
pub struct HelperDummy {
    pub bank_id: u32, // maybe CRC?
    pub transform: ColumnMatrix3x4<f32>,
    pub bone_index: i32,
}

#[derive(Debug)]
pub struct BoneNames {
    indices: Vec<u16>,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct Keyframe {
    pub rotation: Quaternion<f32>,
    pub translation: Vector4<f32>,
    pub scale: Vector4<f32>,
}

#[derive(Debug)]
pub struct Bone {
    pub name_crc: u32,
    pub parent: i32,
    pub original_child_count: i32,
    pub matrix: ColumnMatrix3x4<f32>,
}

#[derive(Debug)]
pub struct BoundingSphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

#[derive(Debug)]
pub struct BoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

#[derive(Debug)]
pub struct Material {
    pub id: u32,
    pub name: String,
    pub decal_id: u32,
    pub base_texture_id: u32,
    pub bumpmap_texture_id: u32,
    pub reflect_texture_id: u32,
    pub illumination_texture_id: u32,
    pub map_flags: u32,
    pub self_illumination: u32,
    pub two_sided: bool,
    pub transparent: bool,
    pub boolean_alpha: bool,
    pub degenerate_triangles: u16,
}

#[derive(Debug)]
pub struct Primitive {
    pub material_index: u32,
    pub repeating_mesh_reps: u32,
    pub bounding_sphere: BoundingSphere,
    pub average_texture_stretch: f32,
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub index_count: u32,
    pub init_flags: u32,
    pub static_block_count: u32,
    pub animated_block_count: u32,
    pub static_blocks: Vec<PrimitiveStaticBlock>,
    pub animated_blocks: Vec<PrimitiveAnimatedBlock>,
    pub pos_bias: [f32; 4],
    pub pos_scale: [f32; 4],
    pub vertex_size: u32,
    pub padding: u32,
    pub vertices: Vec<Vertex>,
    pub index_buffer: Vec<u8>,
    pub cloth_primitives: Vec<ClothPrimitive>,
}

pub const INIT_FLAG_POINT_SPRITE: u32 = 1;
pub const INIT_FLAG_EXTRA_VECTOR: u32 = 2;
pub const INIT_FLAG_PACKED: u32 = 4;
pub const INIT_FLAG_PACKED_POS: u32 = 10;

#[derive(Debug)]
pub struct PrimitiveBlock {
    pub primitive_count: u32,
    pub start_index: u32,
    pub is_strip: bool,
    pub change_flags: u8,
    pub degenerate_triangles: bool,
}

#[derive(Debug)]
pub struct PrimitiveStaticBlock {
    pub base: PrimitiveBlock,
    pub material_index: u32,
}

#[derive(Debug)]
pub struct PrimitiveAnimatedBlock {
    pub base: PrimitiveBlock,
    pub vertex_count: u32,
    pub bones_per_vertex: u16,
    pub paletted_flag: bool,
    pub groups: Vec<u8>,
}

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub uv: Vector2<f32>,
}

#[derive(Debug)]
pub struct ClothPrimitive {
    pub cloth_id: u32,
    pub material_id: u32,
    pub cloth_data_len: u32,
    pub unknown1: u32,
    pub constraints: Vec<ParticleConstraint>,
    pub particles: Particles,
    pub triangle_indices: Vec<[u16; 3]>,
    pub quad_indices: Vec<[u16; 4]>,
    pub render_vertices: Vec<[u16; 2]>,
    pub non_sim_positions: Vec<[f32; 3]>,
    pub indexed_texture_coords: Vec<[f32; 2]>,
    pub particle_indices: Vec<i32>,
    pub vertex_indices: Vec<i32>,
    pub average_patch_size: f32,
    pub bezier_enabled: bool,
    pub export_particles: HashMap<String, u32>,
}

#[derive(Debug)]
pub struct Particles {
    pub timestep: f32,
    pub timestep_changed: f32,
    pub timestep_multipler: f32,
    pub count: u32,
    pub positions: Vec<Vector3<f32>>,
    pub sim_alphas: Vec<f32>,
    pub gravity_strength: f32,
    pub wind_strength: f32,
    pub dragging_enabled: bool,
    pub dragging_rotational: bool,
    pub dragging_strength: f32,
    pub acceleration_enabled: bool,
    pub global_damping: f32,
}

#[derive(Debug)]
pub enum ParticleConstraint {
    Repeat(u32),
    RepeatEnd,
    Distance { ids: [u32; 2], distance: f32, strength: f32 },
    DistanceMinMax { ids: [u32; 2], min: f32, max: f32 },
    Unbend { ids: [u32; 3], strength: f32 },
    SphereCollision { center: Vector3<f32>, radius: f32 },
}

impl Model {
    pub fn decode(mut data: &[u8], info: &BigMeshInfo) -> Option<Model> {
        // println!("{:#?}", info);

        let name = data.parse_str_until_nul()?.to_owned();
        let animated = data.parse_u8()? > 0;
        let bounding_sphere = Self::decode_bounding_sphere(&mut data)?;
        let bounding_box = Self::decode_bounding_box(&mut data)?;
        let helper_points_count = data.parse_u16_le()?;
        let helper_dummies_count = data.parse_u16_le()?;
        let helper_names_uncompressed = data.parse_u32_le()?;
        let _padding = data.parse_u16_le()?;

        // println!("name {:?}", name);
        // println!("animated {:?}", animated);
        // println!("bounding_sphere {:?}", bounding_sphere);
        // println!("bounding_box {:?}", bounding_box);
        // println!("helper_points_count {:?}", helper_points_count);
        // println!("helper_dummies_count {:?}", helper_dummies_count);
        // println!("helper_names_uncompressed {:?}", helper_names_uncompressed);

        let helper_points = Self::decode_helper_points(&mut data, helper_points_count)?;
        // println!("helper_points {:?}", helper_points);
        let helper_dummies = Self::decode_helper_dummies(&mut data, helper_dummies_count)?;
        // println!("helper_dummies {:?}", helper_dummies);
        let (helper_dummy_names, helper_point_names) = Self::decode_helper_names(&mut data, helper_names_uncompressed)?;
        // println!("helper_dummy_names {:?}", helper_dummy_names);
        // println!("helper_point_names {:?}", helper_point_names);

        let material_count = data.parse_u32_le()?;
        let primitive_count = data.parse_u32_le()?;
        let bone_count = data.parse_u32_le()?;
        let bone_names_len = data.parse_u32_le()?;
        let cloth = data.parse_u8()?;
        let unknown4 = data.parse_u16_le()?;
        let unknown5 = data.parse_u16_le()?;

        // println!("material_count {:?}", material_count);
        // println!("primitive_count {:?}", primitive_count);
        // println!("bone_count {:?}", bone_count);
        // println!("bone_names_len {:?}", bone_names_len);
        // println!("cloth {:?}", cloth);
        // println!("unknown4 {:?}", unknown4);
        // println!("unknown5 {:?}", unknown5);

        let bone_names = Self::decode_bone_names(&mut data, bone_count, bone_names_len)?;
        // println!("bone_names {:#?}", bone_names);
        let bones = Self::decode_bones(&mut data, bone_count)?;
        // println!("bones {:#?}", bones);
        let bone_keyframes = Self::decode_bone_keyframes(&mut data, bone_count)?;
        // println!("bone_keyframes {:#?}", bone_keyframes);
        let bone_transforms = Self::decode_bone_transforms(&mut data, bone_count)?;
        // println!("bone_transforms {:#?}", bone_transforms);


        let transform_matrix = data.parse_colmatrix3x4_f32_le()?;

        // println!("transform_matrix {:?}", transform_matrix);

        let mut materials = Vec::with_capacity(material_count as usize);

        for _ in 0..material_count as usize {
            materials.push(Self::decode_material(&mut data)?);
        }

        // println!("materials {:#?}", materials);

        let mut primitives = Vec::with_capacity(primitive_count as usize);

        for _ in 0..primitive_count {
            primitives.push(Self::decode_primitive(&mut data)?);
        }

        // println!("primitives {:#?}", primitives);

        Some(Model {
            name,
            animated,
            bounding_sphere,
            bounding_box,
            helper_points,
            helper_dummies,
            helper_point_names,
            helper_dummy_names,
            material_count,
            primitive_count,
            bone_names,
            bones,
            bone_keyframes,
            bone_transforms,
            transform_matrix,
            cloth,
            unknown4,
            unknown5,
            materials,
            primitives,
        })
    }

    fn decode_bounding_sphere(data: &mut &[u8]) -> Option<BoundingSphere> {
        let center: Vector3<f32> = data.parse_vector3_f32_le()?.into();
        let radius = data.parse_f32_le()?;
        Some(BoundingSphere { center, radius })
    }

    fn decode_bounding_box(data: &mut &[u8]) -> Option<BoundingBox> {
        let min: Vector3<f32> = data.parse_vector3_f32_le()?.into();
        let max: Vector3<f32> = data.parse_vector3_f32_le()?.into();
        Some(BoundingBox { min, max })
    }

    fn decode_helper_points(data: &mut &[u8], helper_points_count: u16) -> Option<Vec<HelperPoint>> {
        let mut helper_points = Vec::with_capacity(helper_points_count as usize);

        if helper_points_count > 0 {
            let data = Self::decode_semi_compressed(data, 20 * helper_points_count as usize)?;

            let mut data = &data[..];

            for _ in 0..helper_points_count {
                let bank_id = data.parse_u32_le()?;
                let point: Vector3<f32> = data.parse_vector3_f32_le()?.into();
                let bone_index = data.parse_i32_le()?;
                helper_points.push(HelperPoint { bank_id, point, bone_index });
            }
        }

        Some(helper_points)
    }

    fn decode_helper_dummies(data: &mut &[u8], helper_dummies_count: u16) -> Option<Vec<HelperDummy>> {
        let mut helper_dummies = Vec::with_capacity(helper_dummies_count as usize);

        if helper_dummies_count > 0 {
            let data = Self::decode_semi_compressed(data, 56 * helper_dummies_count as usize)?;

            let mut data = &data[..];

            for _ in 0..helper_dummies_count {
                let bank_id = data.parse_u32_le()?;
                let transform = data.parse_colmatrix3x4_f32_le()?;
                let bone_index = data.parse_i32_le()?;
                helper_dummies.push(HelperDummy { bank_id, transform, bone_index });
            }
        }

        Some(helper_dummies)
    }

    fn decode_helper_names(data: &mut &[u8], helper_names_uncompressed: u32) -> Option<(Vec<u8>, Vec<u8>)> {
        if helper_names_uncompressed > 0  {
            let helper_names = Self::decode_semi_compressed(data, helper_names_uncompressed as usize)?;

            let mut helper_names = &helper_names[..];

            let helper_point_index_size = helper_names.parse_u16_le()?;

            let helper_point_names = helper_names.advance(helper_point_index_size.saturating_sub(2) as usize)?.to_owned();
            let helper_dummy_names = helper_names.advance(helper_names_uncompressed.saturating_sub(helper_point_index_size as u32) as usize)?.to_owned();

            Some((helper_point_names, helper_dummy_names))
        } else {
            Some((vec![], vec![]))
        }
    }

    fn decode_bone_names(
        data: &mut &[u8],
        bone_count: u32,
        bone_names_len: u32
    ) -> Option<BoneNames> {
        if bone_count > 0 {
            if bone_names_len > 0 {
                let indices_data = Self::decode_semi_compressed(data, 2 * (bone_count - 1) as usize)?;
                let mut indices_data = &indices_data[..];
                let indices = (0..bone_count - 1)
                    .map(|_| indices_data.parse_u16_le())
                    .collect::<Option<Vec<_>>>()?;

                let data = Self::decode_semi_compressed(data, bone_names_len as usize)?;

                Some(BoneNames { indices, data })
            } else {
                Some(BoneNames { indices: vec![], data: vec![] })
            }
        } else {
            Some(BoneNames { indices: vec![], data: vec![] })
        }
    }

    fn decode_bones(data: &mut &[u8], bone_count: u32) -> Option<Vec<Bone>> {
        let mut bones = Vec::with_capacity(bone_count as usize);

        if bone_count > 0 {
            let bone_data = Self::decode_semi_compressed(data, 60 * bone_count as usize)?;
            let mut bone_data = bone_data.as_slice();

            for _ in 0 .. bone_count as usize {
                let name_crc = bone_data.parse_u32_le()?;
                let parent = bone_data.parse_i32_le()?;
                let original_child_count = bone_data.parse_i32_le()?;
                let matrix = bone_data.parse_colmatrix3x4_f32_le()?;
                bones.push(Bone { name_crc, parent, original_child_count, matrix })
            }
        }

        Some(bones)
    }

    fn decode_bone_keyframes(data: &mut &[u8], bone_count: u32) -> Option<Vec<Keyframe>> {
        let mut bone_keyframes = Vec::with_capacity(bone_count as usize);

        if bone_count > 0 {
            let bone_keyframe_data = Self::decode_semi_compressed(data, 48 * bone_count as usize)?;
            let mut bone_keyframe_data = bone_keyframe_data.as_slice();

            for _ in 0 .. bone_count as usize {
                let rotation = bone_keyframe_data.parse_quaternion_f32_le()?;
                let translation: Vector4<f32> = bone_keyframe_data.parse_vector4_f32_le()?.into();
                let scale: Vector4<f32> = bone_keyframe_data.parse_vector4_f32_le()?.into();
                bone_keyframes.push(Keyframe { rotation, translation, scale });
            }
        }

        Some(bone_keyframes)
    }

    fn decode_bone_transforms(data: &mut &[u8], bone_count: u32) -> Option<Vec<ColumnMatrix4<f32>>> {
        let mut bone_transforms = Vec::with_capacity(bone_count as usize);

        if bone_count > 0 {
            let bone_transform_data = Self::decode_semi_compressed(data, 64 * bone_count as usize)?;
            let mut bone_transform_data = bone_transform_data.as_slice();

            for _ in 0 .. bone_count as usize {
                bone_transforms.push(bone_transform_data.parse_colmatrix4_f32_le()?);
            }
        }

        Some(bone_transforms)
    }

    fn decode_material(data: &mut &[u8]) -> Option<Material> {
        let id = data.parse_u32_le()?;
        let name = data.parse_str_until_nul()?.to_owned();
        let decal_id = data.parse_u32_le()?;
        let base_texture_id = data.parse_u32_le()?;
        let bumpmap_texture_id = data.parse_u32_le()?;
        let reflect_texture_id = data.parse_u32_le()?;
        let illumination_texture_id = data.parse_u32_le()?;
        let map_flags = data.parse_u32_le()?;
        let self_illumination = data.parse_u32_le()?;
        let two_sided = data.parse_u8()? != 0;
        let transparent = data.parse_u8()? != 0;
        let boolean_alpha = data.parse_u8()? != 0;
        let degenerate_triangles = data.parse_u16_le()?; // ?

        Some(Material {
            id,
            name,
            decal_id,
            base_texture_id,
            bumpmap_texture_id,
            reflect_texture_id,
            illumination_texture_id,
            map_flags,
            self_illumination,
            two_sided,
            transparent,
            boolean_alpha,
            degenerate_triangles,
        })
    }

    fn decode_primitive(data: &mut &[u8]) -> Option<Primitive> {
        let material_index = data.parse_u32_le()?;
        let repeating_mesh_reps = data.parse_u32_le()?;
        let bounding_sphere = Self::decode_bounding_sphere(data)?;
        let average_texture_stretch = data.parse_f32_le()?;
        let _static_block_count = data.parse_u32_le()?;
        let _animated_block_count = data.parse_u32_le()?;
        let vertex_count = data.parse_u32_le()?;
        let triangle_count = data.parse_u32_le()?;
        let index_count = data.parse_u32_le()?;
        let init_flags = data.parse_u32_le()?;

        // The game takes init_flags and  copies all but the third bit into init_flags. It checks if animated_block_count is not equal to zero and sets vertex_stream_flags to 1. It checks if the second bit in init_flags is set and enables the second bit in vertex_stream_flags. It checks if the 5th bit in confusing_bits is set and sets the 3rd bit in vertex_stream_flags.

        // let init_flags = init_flags & 0xfffffffb;
        // let mut vertex_stream_flags = if animated_block_count != 0 { 1 } else { 0 };
        // if init_flags & 2 != 0 { vertex_stream_flags |= 2 }
        // if init_flags & 16 != 0 { vertex_stream_flags |= 4 }

        let static_block_count = data.parse_u32_le()?;
        let animated_block_count = data.parse_u32_le()?;

        let mut static_blocks = Vec::with_capacity(static_block_count as usize);

        for _ in 0..static_block_count {
            static_blocks.push(Self::decode_static_block(data)?);
        }

        // println!("static_blocks {:?}", static_blocks);

        let mut animated_blocks = Vec::with_capacity(animated_block_count as usize);

        for _ in 0..animated_block_count {
            animated_blocks.push(Self::decode_animated_block(data)?);
        }

        // println!("animated_blocks {:?}", animated_blocks);

        let pos_scale = data.parse_vector4_f32_le()?;
        let pos_bias = data.parse_vector4_f32_le()?;

        let vertex_size = data.parse_u32_le()?;
        let padding = data.parse_u32_le()?;

        let mesh_level_size = vertex_count as usize * vertex_size as usize;

        let vertex_data = Self::decode_semi_compressed(
            data,
            mesh_level_size + mesh_level_size * repeating_mesh_reps as usize,
        )?;

        let mut vertex_data = vertex_data.as_slice();

        let mut vertices = Vec::with_capacity(vertex_count as usize);

        for _ in 0 .. vertex_count {
            vertices.push(Self::decode_vertex(
                &mut vertex_data,
                animated_block_count > 0,
                init_flags,
                pos_scale,
                pos_bias
            )?);
        }

        // println!("vertices {:?}", vertices);

        let index_level_size = 2 * index_count as usize;

        let index_buffer = Self::decode_semi_compressed(
            data,
            index_level_size + index_level_size * repeating_mesh_reps as usize,
        )?;

        let cloth_primitives_count = data.parse_u32_le()?;

        let mut cloth_primitives = Vec::with_capacity(cloth_primitives_count as usize);

        for _ in 0 .. cloth_primitives_count {
            cloth_primitives.push(Self::decode_cloth_primitive(data)?);
        }

        Some(Primitive {
            material_index,
            repeating_mesh_reps,
            bounding_sphere,
            average_texture_stretch,
            vertex_count,
            triangle_count,
            index_count,
            init_flags,
            static_block_count,
            animated_block_count,
            static_blocks,
            animated_blocks,
            pos_scale,
            pos_bias,
            vertex_size,
            padding,
            vertices,
            index_buffer,
            cloth_primitives,
        })
    }

    fn decode_base_block(data: &mut &[u8]) -> Option<PrimitiveBlock> {
        let primitive_count = data.parse_u32_le()?;
        let start_index = data.parse_u32_le()?;
        let is_strip = data.parse_u8()? != 0;
        let change_flags = data.parse_u8()?;
        let degenerate_triangles = data.parse_u8()? != 0;
        Some(PrimitiveBlock {
            primitive_count,
            start_index,
            is_strip,
            change_flags,
            degenerate_triangles,
        })
    }

    fn decode_static_block(data: &mut &[u8]) -> Option<PrimitiveStaticBlock> {
        let base = Self::decode_base_block(data)?;
        let material_index = data.parse_u32_le()?;
        Some(PrimitiveStaticBlock { base, material_index })
    }

    fn decode_animated_block(data: &mut &[u8]) -> Option<PrimitiveAnimatedBlock> {
        let base = Self::decode_base_block(data)?;
        let vertex_count = data.parse_u32_le()?;
        let bones_per_vertex = data.parse_u16_le()?;
        let paletted_flag = data.parse_u8()? != 0;

        let groups_len = data.parse_u8()?;
        let groups = data.advance(groups_len as usize)?.to_owned();

        Some(PrimitiveAnimatedBlock {
            base,
            vertex_count,
            bones_per_vertex,
            paletted_flag,
            groups,
        })
    }

    fn decode_vertex(
        data: &mut &[u8],
        animated: bool,
        init_flags: u32,
        pos_scale: [f32; 4],
        pos_bias: [f32; 4],
    ) -> Option<Vertex> {


        let mut pos: Vector3<f32> = if
            (animated && ((init_flags & INIT_FLAG_PACKED == 0) && (init_flags & INIT_FLAG_PACKED_POS != 0))) &&
            (!animated && ((init_flags & INIT_FLAG_PACKED == 0) || (init_flags & INIT_FLAG_PACKED_POS != 0)))
        {
            data.parse_vector3_f32_le()?.into()
        } else {
            let arr: [f32; 3] = data.parse_vector3_packed()?.into();
            arr.into()
        };

        pos.x = pos.x * pos_scale[0] + pos_bias[0];
        pos.y = pos.y * pos_scale[1] + pos_bias[1];
        pos.z = pos.z * pos_scale[2] + pos_bias[2];

        eprintln!("pos {:?}", pos);

        if animated {
            let _d3dcolor_1 = data.parse_u32_le()?;
            let _d3dcolor_2 = data.parse_u32_le()?;
            eprintln!("Warning: Unused d3dcolors {:0>8x?} {:0>8x?}", _d3dcolor_1, _d3dcolor_2);
        }

        let normal = if init_flags & INIT_FLAG_PACKED == 0 {
            data.parse_vector3_f32_le()?.into()
        } else {
            let arr: [f32; 3] = data.parse_vector3_packed()?.into();
            arr.into()
        };

        let uv = if init_flags & INIT_FLAG_PACKED == 0 {
            data.parse_vector2_f32_le()?.into()
        } else {
            let uv = data.parse_vector2_f16_le()?;
            [uv[0].into(), uv[1].into()].into()
        };

        if init_flags & INIT_FLAG_EXTRA_VECTOR != 0 {
            let _extra: Vector4<f32> = if init_flags & INIT_FLAG_PACKED == 0 {
                data.parse_vector4_f32_le()?.into()
            } else {
                let x = data.parse_u16_le()? as f32 * 0.00003052;
                let y = data.parse_u16_le()? as f32 * 0.00003052;
                let z = data.parse_u16_le()? as f32 * 0.00003052;
                let w = data.parse_u16_le()? as f32 * 0.00003052;
                [x, y, z, w].into()
            };
            eprintln!("Warning: Unused extra vector {:?}", _extra);
        }

        Some(Vertex { pos, normal, uv })
    }

    fn decode_cloth_primitive(data: &mut &[u8]) -> Option<ClothPrimitive> {
        let cloth_id: u32 = data.parse_u32_le()?;
        let material_id: u32 = data.parse_u32_le()?;

        let cloth_data_len: u32 = data.parse_u32_le()?;
        let cloth_data = Self::decode_semi_compressed(data, cloth_data_len as usize)?;
        let mut cloth_data = cloth_data.as_slice();

        let unknown1: u32 = cloth_data.parse_u32_le()?;

        let constraints_data_len: u32 = cloth_data.parse_u32_le()?;
        let mut constraints_data = cloth_data.advance(constraints_data_len as usize)?;
        let mut constraints = Vec::new();

        while let Some(constraint) = Self::decode_constraint(&mut constraints_data) {
            constraints.push(constraint);
        }

        let particles: Particles = Self::decode_particles(&mut cloth_data)?;

        let triangle_count: u32 = cloth_data.parse_u32_le()?;
        let mut triangle_indices = Vec::with_capacity(triangle_count as usize);

        for _ in 0 .. triangle_count {
            triangle_indices.push([
                cloth_data.parse_u16_le()?, cloth_data.parse_u16_le()?, cloth_data.parse_u16_le()?,
            ]);
        }

        let quad_count: u32 = cloth_data.parse_u32_le()?;
        let mut quad_indices = Vec::with_capacity(quad_count as usize);

        for _ in 0 .. quad_count {
            quad_indices.push([
                cloth_data.parse_u16_le()?, cloth_data.parse_u16_le()?,
                cloth_data.parse_u16_le()?, cloth_data.parse_u16_le()?,
            ]);
        }

        let render_vertices_count = cloth_data.parse_u32_le()?;
        let mut render_vertices: Vec<[u16; 2]> = Vec::with_capacity(render_vertices_count as usize);

        for _ in 0 .. render_vertices_count {
            render_vertices.push([cloth_data.parse_u16_le()?, cloth_data.parse_u16_le()?]);
        }

        let non_sim_count: u32 = cloth_data.parse_u32_le()?;
        let mut non_sim_positions: Vec<[f32; 3]> = Vec::with_capacity(non_sim_count as usize);

        for _ in 0 .. non_sim_count {
            non_sim_positions.push(cloth_data.parse_vector3_f32_le()?);
        }

        let indexed_texture_coords_count: u32 = cloth_data.parse_u32_le()?;
        let mut indexed_texture_coords: Vec<[f32; 2]> = Vec::with_capacity(indexed_texture_coords_count as usize);

        for _ in 0 .. indexed_texture_coords_count {
            indexed_texture_coords.push(cloth_data.parse_vector2_f32_le()?);
        }

        let particle_indices_count: u32 = cloth_data.parse_u32_le()?;
        let mut particle_indices: Vec<i32> = Vec::with_capacity(particle_indices_count as usize);

        for _ in 0 .. particle_indices_count {
            particle_indices.push(cloth_data.parse_i32_le()?);
        }

        let vertex_indices_count: u32 = cloth_data.parse_u32_le()?;
        let mut vertex_indices: Vec<i32> = Vec::with_capacity(vertex_indices_count as usize);

        for _ in 0 .. vertex_indices_count {
            vertex_indices.push(cloth_data.parse_i32_le()?);
        }

        let average_patch_size: f32 = cloth_data.parse_f32_le()?;
        let bezier_enabled: bool = cloth_data.parse_u8()? != 0;

        let export_particles_count: u32 = cloth_data.parse_u32_le()?;
        let mut export_particles: HashMap<String, u32> = HashMap::with_capacity(export_particles_count as usize);

        for _ in 0 .. export_particles_count {
            let name = cloth_data.parse_str_with_u32_le_prefix()?.to_owned();
            let id = cloth_data.parse_u32_le()?;
            export_particles.insert(name, id);
        }

        Some(ClothPrimitive {
            cloth_id,
            material_id,
            cloth_data_len,
            unknown1,
            constraints,
            particles,
            triangle_indices,
            quad_indices,
            render_vertices,
            non_sim_positions,
            indexed_texture_coords,
            particle_indices,
            vertex_indices,
            average_patch_size,
            bezier_enabled,
            export_particles,
        })
    }

    fn decode_constraint(data: &mut &[u8]) -> Option<ParticleConstraint> {
        let type_id = data.parse_u32_le()?;
        let repeat_count = data.parse_u32_le()?;
        let size = data.parse_u32_le()?;
        match (type_id, size) {
            (0, 0) => Some(ParticleConstraint::Repeat(repeat_count)),
            (1, 0) => Some(ParticleConstraint::RepeatEnd),
            (2, 16) => {
                let ids = [data.parse_u32_le()?, data.parse_u32_le()?];
                let distance = data.parse_f32_le()?;
                let strength = data.parse_f32_le()?;
                Some(ParticleConstraint::Distance { ids, distance, strength })

            }
            (3, 16) => {
                let ids = [data.parse_u32_le()?, data.parse_u32_le()?];
                let min = data.parse_f32_le()?;
                let max = data.parse_f32_le()?;
                Some(ParticleConstraint::DistanceMinMax { ids, min, max })
            }
            (4, 16) => {
                let ids = [data.parse_u32_le()?, data.parse_u32_le()?, data.parse_u32_le()?];
                let strength = data.parse_f32_le()?;
                Some(ParticleConstraint::Unbend { ids, strength })
            },
            (5, 16) => {
                let center = data.parse_vector3_f32_le()?.into();
                let radius = data.parse_f32_le()?;
                Some(ParticleConstraint::SphereCollision { center, radius })
            },
            _ => {
                None
            }
        }
    }

    fn decode_particles(data: &mut &[u8]) -> Option<Particles> {
        let timestep: f32 = data.parse_f32_le()?;
        let timestep_changed: f32 = data.parse_f32_le()?;
        let timestep_multipler: f32 = data.parse_f32_le()?;
        let count: u32 = data.parse_u32_le()?;

        let mut positions: Vec<Vector3<f32>> = Vec::with_capacity(count as usize);

        for _ in 0 .. count {
            positions.push(data.parse_vector3_f32_le()?.into());
        }

        let mut sim_alphas: Vec<f32> = Vec::with_capacity(count as usize);

        for _ in 0 .. count {
            sim_alphas.push(data.parse_f32_le()?);
        }

        let gravity_strength: f32 = data.parse_f32_le()?;
        let wind_strength: f32 = data.parse_f32_le()?;
        let dragging_enabled: bool = data.parse_u8()? != 0;
        let dragging_rotational: bool = data.parse_u8()? != 0;
        let dragging_strength: f32 = data.parse_f32_le()?;
        let acceleration_enabled: bool = data.parse_u8()? != 0;
        let global_damping: f32 = data.parse_f32_le()?;

        Some(Particles {
            timestep,
            timestep_changed,
            timestep_multipler,
            count,
            positions,
            sim_alphas,
            gravity_strength,
            wind_strength,
            dragging_enabled,
            dragging_rotational,
            dragging_strength,
            acceleration_enabled,
            global_damping,
        })
    }

    // TODO: Move this to crate::shared if its used in other files.
    fn decode_semi_compressed(data: &mut &[u8], mut size: usize) -> Option<Vec<u8>> {
        let compressed_len = data.parse_u16_le()?;

        let compressed_len = if compressed_len == 0xFFFF {
            data.parse_u32_le()? as usize
        } else {
            compressed_len as usize
        };

        let mut uncompressed = Vec::with_capacity(size);

        if compressed_len > 0 {
            let input = data.advance(compressed_len as usize)?;

            let out = match crate::lzo::decompress(&input, size) {
                Err(_e) => return None,
                Ok(r) => r,
            };

            size -= out.len() as usize;

            uncompressed.extend(&out);
        }

        if size > 0 {
            uncompressed.extend(data.advance(size)?);
        }

        Some(uncompressed)
    }
}