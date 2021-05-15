use std::io::Read;

use crate::{Bytes,BigMeshInfo};

/// Mesh format.
///
/// ## Format Description
///
/// A placeholder description from <http://fabletlcmod.com>:
///
/// ```txt
///  Tagged Model Format
///
///  3DMF: 3D Mesh File
///      3DRT: (File Size of all Chunks)
///      MTLS: Material List (File Size of All MTRL Chunks)
///          MTRL: Material Description
///          MTLE: Extended Material Properties
///          MMAP: Mapping Info
///      SUBM: Sub-Mesh
///          TRFM: Transformation Matrix
///          PRIM: Primitive
///              TRIS: Triangle List
///              SMTH: Smoothing Groups
///              VERT: Vertex List
///              UNIV: Unique Vertex Information
///              VGRP: Vertex Group
///          BONE: Bone
///          CLTH: Cloth
///      HLPR: Helpers
///          HDMY: Dummy Object
///          HPNT: Helper Point
///          HCVL: Convex Volume
///
///  Compiled Model Format
///
/// char         NullTerminatedString[x];
/// byte         SkeletonPresent;
/// float        floats[10]; //Model Origin?? Listed in .big Sub-header...
/// word         HPNT_Count;
/// word         HDMY_Count;
/// dword        HLPR_Index_Uncompressed;
/// word			padding;
/// word			HPNT_Compressed;
/// Helper Points[HPNT_Count];
///   float         Matrix[4]; //No Rotation
///   long          hierarchy;
/// word			HDMY_Compressed;
/// Helper Dummies[HDMY_Count];
///   float        Matrix[13];
///   long         hierarchy;
/// word			HLPR_Index_Compressed;
/// word			HPNT_IndexSize;
/// char			HPNT_Index[HPNT_IndexSize-2]; //Subtract the size
/// char		    HDMY_Index[HLPR_Index_Uncompressed-HPNT_IndexSize]; //Rest of helper index deduced
/// dword        NumberMaterials;
/// dword        NumberSurfaces;
/// dword        NumberBones;
/// dword        SizeOfBoneIndex;
/// byte         Unknown;
/// word         Unknown;
/// word         Unknown;
/// word         Compressed;
/// word         Bone_Index_Reference[NumberBones-1];
/// word         BoneIndexCompressed;
/// char         BoneIndex[SizeOfBoneIndex];
/// word         CompressedSize;
/// Bones SUB CHUNK 1[NumberBones];
/// word					CompressedSize;
/// Bones SUB CHUNK 2[NumberBones];
/// word					CompressedSize;
/// Bones SUB CHUNK 3[NumberBones];
/// float					Matrix[12]; //Transform Matrix
///
///  Bones
///      SUB CHUNK 1
///          long					Index;
///          long					Parent;
///          long					nChild;
///          float					Matrix[12];
///      SUB CHUNK 2
///          float					Matrix[12];
///      SUB CHUNK 3
///          float					Matrix[16];
///
///  Material List
///      dword					Material_Index;
///      char					NullTerminatedString[x];
///      dword					Padding;
///      dword					BASE_Texture_ID; //From Texture.big
///      dword					BUMPMAP_Texture_ID; //From Texture.big
///      dword					REFLECT_Texture_ID; //From Texture.big
///      dword					Unknown;
///      dword					Max_Texture_Layers;
///      dword					Glow_Strength;
///      byte					Unknown;
///      byte					Alpha_Enabled;
///      byte					Unknown;
///      word					Ignored; //For degenerate Tri's
///
///  Sub-Mesh
///  dword							Hierarchy;
///  dword							DestroyableMeshLevels;
///  float							floats[5];
///  dword							nFaceVertexIndices;
///  dword							nFaceVertexIndices_BoneIndice;
///  dword							nVerts;
///  dword							nFaces;
///  dword							nSourceVerts;
///  dword							Unknown;
///  dword							Unknown;
///  dword							Unknown;
///  struct structMTRLnFaceVertexIndices			FaceIndices[nFaceVertexIndices];
///  struct structMTRLnFaceVertexIndicesBoneIndice		Face_BoneIndices[nFaceVertexIndices_BoneIndice];
///  float							floats[8];
///  dword							sVert;
///  dword							padding;
///  //Start of Mesh
///
///  Quick notes on sVert “Size Vertice Blocks”….
///
///  20 - 28byte float coords, bones, packed normals, s11e4 tu tv
///  6 - 28byte packed coords, bones, packed normals, s11e4 tu, tv, dword[2]
///  4 - 36byte float coords, float normals, float tu tv, dword meshlevel
///  22 - 36byte float coords, bones, packed normals, s11e4 tu,tv, dword[2]
///  4 - 12byte packed coords, packed normals, s11e4 tu tv
///  4 - 20byte packed coords, bones, packed normals, s11e4 tu tv
///  20 - 20byte float coords, packed normals, s11e4 tu tv
///  Dynamic Clothing
///
///   struct CLTH
///  {
///  	//DWORD				SubMesh_ClothCount;
///  	DWORD				Cloth_ID;
///  	DWORD				??_ID; //possibly material ID
///  	DWORD				sChunk; //Size of full clothing data
///  	DWORD				Unknown5;
///  	DWORD				sDistanceIndice;
///  	CLTH_Distance*			DistanceIndice;//[sDistanceIndice/28]; //Distance between two particles
///  	float				Unknown8;
///  	float				Unknown9;
///  	float				Unknown10;
///  	DWORD				sParticleIndice;
///  	CLTH_Particle*			ParticleIndice;//[sParticleIndice];
///  	float*				ParticleAlphaIndice;//[sParticleIndice]; //How "free" they are. 0.0 = Static and gets a duped //  entry in verts
///  	DWORD				Unknown11;
///  	float				WindStrength; //strength
///  	char				EnableDragging; //enable
///  	char				RotationalDragging; //rotational
///  	float				StrengthDragging; //strength
///  	char				EnableAcceleration; //enable
///  	float				AccelerationDampening; //damping
///  	DWORD				nTriIndice;
///  	CLTH_Tri*			TriIndice;//[nTriIndice] Particles+"Unique" Verts
///  	DWORD				Unknown12; // looks like padding it
///  	DWORD				sTexIndice;
///  	CLTH_Tex*			TexIndice;//[sTexIndice]; //v1 = Particle/"unique" Vert, v2 = TexIndice
///  	DWORD				sVertexIndice;
///  	CLTH_Vertex*			VertexIndice;//[sVertexIndice];
///  	DWORD				sTexCoordIndice;
///  	CLTH_TexCoord*			TexCoordIndice;//[sTexCoordIndice];
///  	DWORD				sParticleMask;
///  	CLTH_PartMask*			ParticleMask;//[sParticleMask]; //Unique Particles in TriIndice
///  	DWORD				sVertMask;
///  	CLTH_VertMask*			Vertmask;//[sVertMask]; //Unique Verts in TriIndice
///  	//9 bytes of padding
///  	// 1 group for particles, 1 for verts
///  	DWORD				VGRPCount; // = Number of Bones
///  	VGRP**				VGRPs;
///  };
/// ```
///
#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub has_skeleton: bool,
    pub model_origin: Vec<f32>,
    pub helper_points_count: u16,
    pub helper_dummies_count: u16,
    pub helper_index_uncompressed: u32,
    pub padding: u16,
    pub helper_points: Vec<MeshHelperPoint>,
    pub helper_dummies: Vec<MeshHelperDummy>,
    // pub hlpr_index_compressed: u16,
    // pub hpnt_index_size: u16,
    // pub hpnt_index: Vec<u8>,
    // pub hdmy_index: Vec<u8>,
    pub material_count: u32,
    pub surface_count: u32,
    pub bone_count: u32,
    pub bone_index_size: u32,
    pub unknown3: u8,
    pub unknown4: u16,
    pub unknown5: u16,
    // pub compressed: u16,
    // pub bone_index_reference: Vec<u16>,
    // pub bone_index_compressed: u16,
    // pub bone_index: Vec<u8>,
    // pub compressed_size: u16,
}

#[derive(Debug)]
pub struct MeshHelperPoint {
    pub matrix: Vec<f32>, // 2x2 matrix?
    pub hierarchy: i32,
}

#[derive(Debug)]
pub struct MeshHelperDummy {
    pub matrix: Vec<f32>, // 13 float matrix?
    pub hierarchy: i32,
}

#[derive(Debug)]
pub struct MeshMaterial {
    pub id: u32,
    pub name: String,
    pub padding: u32,
    pub base_texture_id: u32,
    pub bumpmap_texture_id: u32,
    pub reflect_texture_id: u32,
    pub unknown_1: u32,
    pub max_texture_layers: u32,
    pub glow_strength: u32,
    pub unknown_2: u8,
    pub alpha_enabled: bool,
    pub unknown_3: u8,
    pub ignored: u16,
}

#[derive(Debug)]
struct MeshBone1 {
    index: i32,
    parent: i32,
    child_count: i32,
    matrix: Vec<f32>, // 12
}

#[derive(Debug)]
struct MeshBone2 {
    matrix: Vec<f32>, // 12
}

#[derive(Debug)]
struct MeshBone3 {
    matrix: Vec<f32>, // 16
}

#[derive(Debug)]
pub struct MeshFaceBoneIndex {
    part_1: Vec<u8>,
    part_2_length: u8,
    part_2: Vec<u8>,
}

#[derive(Debug)]
pub struct MeshSubMesh {
    hierarchy: u32,
    destroyable_mesh_levels: u32,
    floats_1: Vec<f32>,
    n_face_vertex_indices: u32,
    n_face_vertex_indices_bone_index: u32,
    n_verts: u32,
    n_faces: u32,
    n_source_verts: u32,
    unknown_1: u32,
    unknown_2: u32,
    unknown_3: u32,
    face_indices: Vec<u8>,
    face_bone_indices: Vec<MeshFaceBoneIndex>,
    floats_2: Vec<f32>,
    s_vert: u32,
    padding: u32,
    // vertex_buffer: Vec<u8>,
}

#[derive(Debug)]
pub struct MeshVertex1 {
    unknown_1: f32,
    unknown_2: f32,
    unknown_3: u16,
    unknown_4: u16,
}

#[derive(Debug)]
pub struct MeshVertex2 {
    unknown: Vec<u8>,
}

pub struct MeshVertex36 {

}

impl Mesh {
    pub fn decode(mut data: &[u8], info: &BigMeshInfo) -> Option<Mesh> {
        println!("{:#?}", info);

        let name = data.grab_str_until_nul()?.to_owned();
        let has_skeleton = data.grab_u8()? > 0;
        let model_origin = (0..10).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
        let helper_points_count = data.grab_u16_le()?;
        let helper_dummies_count = data.grab_u16_le()?;
        let helper_index_uncompressed = data.grab_u32_le()?;
        let padding = data.grab_u16_le()?;

        // println!("name {:?}", name);
        // println!("has_skeleton {:?}", has_skeleton);
        // println!("model_origin {:?}", model_origin);
        // println!("helper_points_count {:?}", helper_points_count);
        // println!("helper_dummies_count {:?}", helper_dummies_count);
        // println!("helper_index_uncompressed {:?}", helper_index_uncompressed);
        // println!("padding {:?}", padding);

        let mut helper_points = Vec::with_capacity(helper_points_count as usize);

        if helper_points_count > 0 {
            let compressed = data.grab_u16_le()?;

            let size = 20 * helper_points_count as usize;
            let data = Self::decode_semi_compressed(&mut data, compressed as usize, size)?;
            let mut data = &data[..];

            for _ in 0..helper_points_count {
                let matrix = (0..4).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
                let hierarchy = data.grab_i32_le()?;
                helper_points.push(MeshHelperPoint { matrix, hierarchy });
            }
        }

        // println!("helper_points {:#?}", helper_points);

        let mut helper_dummies = Vec::with_capacity(helper_dummies_count as usize);

        if helper_dummies_count > 0 {
            let compressed = data.grab_u16_le()?;
            // println!("helper_dummies compressed {:?}", compressed);
            let size = 56 * helper_dummies_count as usize;
            let data = Self::decode_semi_compressed(&mut data, compressed as usize, size)?;
            let mut data = &data[..];

            for _ in 0..helper_dummies_count {
                let matrix = (0..13).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
                let hierarchy = data.grab_i32_le()?;
                helper_dummies.push(MeshHelperDummy { matrix, hierarchy });
            }
        }

        // println!("helper_dummies {:#?}", helper_dummies);

        let (helper_dummy_index, helper_point_index) = if helper_index_uncompressed > 0  {
            let helper_index_compressed = data.grab_u16_le()?;
            // println!("helper_index_compressed {:?}", helper_index_compressed);
            let helper_index = Self::decode_semi_compressed(&mut data, helper_index_compressed as usize, helper_index_uncompressed as usize)?;
            let mut helper_index = &helper_index[..];
            // println!("helper_index {:X?}", helper_index);

            let helper_point_index_size = helper_index.grab_u16_le()?;
            // println!("helper_point_index_size {:?}", helper_point_index_size);
            let helper_point_index = helper_index.grab(helper_point_index_size.saturating_sub(2) as usize)?.to_owned();
            // println!("helper_point_index {:X?}", helper_point_index);
            // let out = crate::lzo::decompress(&helper_point_index[1..], helper_point_index_size as usize).unwrap();
            // println!("helper_point_index {:?}", helper_point_index);
            let helper_dummy_index = helper_index.grab(helper_index_uncompressed.saturating_sub(helper_point_index_size as u32) as usize)?.to_owned();

            // println!("helper_dummy_index {:?} {:X?}", helper_dummy_index.len(), helper_dummy_index);

            (helper_point_index, helper_dummy_index)
        } else {
            (Vec::new(), Vec::new())
        };
        // println!("helper_dummy_index {:?}", helper_dummy_index);

        let material_count = data.grab_u32_le()?;
        let surface_count = data.grab_u32_le()?;
        let bone_count = data.grab_u32_le()?;
        // NOTE: Are these values only present if bone_count > 0?
        let bone_index_size = data.grab_u32_le()?;
        let unknown3 = data.grab_u8()?;
        let unknown4 = data.grab_u16_le()?;
        let unknown5 = data.grab_u16_le()?;

        // println!("material_count {:?}", material_count);
        // println!("surface_count {:?}", surface_count);
        // println!("bone_count {:?}", bone_count);
        // println!("data now {:X?}", &data[..32]);
        // println!("bone_index_size {:?}", bone_index_size);
        // println!("unknown3 {:?}", unknown3);
        // println!("unknown4 {:?}", unknown4);
        // println!("unknown5 {:?}", unknown5);

        if bone_count > 0 {
            if bone_index_size > 0 {
                let compressed = data.grab_u16_le()? as usize;
                let index_data = Self::decode_semi_compressed(&mut data, compressed, 2 * (bone_count - 1) as usize)?;
                let mut index_data = &index_data[..];
                let bone_index_reference = (0..bone_count - 1)
                    .map(|_| index_data.grab_u16_le())
                    .collect::<Option<Vec<_>>>()?;

                // println!("bone_index_reference {:?}", bone_index_reference);

                let bone_index_compressed = data.grab_u16_le()? as usize;

                // println!("bone_index_compressed {:?}", bone_index_compressed);

                let bone_index = Self::decode_semi_compressed(&mut data, bone_index_compressed, bone_index_size as usize)?;

                // println!("bone_index {:?}", bone_index);
            }

            // let compressed_size = data.grab_u16_le()?;

            // let _ = data.grab_u16_le()?;

            // println!("compressed_size {:?}", compressed_size);

            let bones_1_compressed = data.grab_u16_le()? as usize;
            // println!("bones_1_compressed {:?}", bones_1_compressed);
            let bones_1_size = 60 * bone_count as usize;
            let bones_1_data = Self::decode_semi_compressed(&mut data, bones_1_compressed, bones_1_size)?;
            let mut bones_1_data = &bones_1_data[..];
            let bones_1 = (0..bone_count).map(|_| Self::decode_bone_1(&mut bones_1_data)).collect::<Option<Vec<_>>>()?;
            // println!("bones_1 {:?}", bones_1);

            let bones_2_compressed = data.grab_u16_le()? as usize;
            // println!("bones_2_compressed {:?}", bones_2_compressed);
            let bones_2_size = 48 * bone_count as usize;
            let bones_2_data = Self::decode_semi_compressed(&mut data, bones_2_compressed, bones_2_size)?;
            let mut bones_2_data = &bones_2_data[..];
            let bones_2 = (0..bone_count).map(|_| Self::decode_bone_2(&mut bones_2_data)).collect::<Option<Vec<_>>>()?;
            // println!("bones_2 {:?}", bones_2);

            let bones_3_compressed = data.grab_u16_le()? as usize;
            // println!("bones_3_compressed {:?}", bones_3_compressed);
            let bones_3_size = 64 * bone_count as usize;
            let bones_3_data = Self::decode_semi_compressed(&mut data, bones_3_compressed, bones_3_size)?;
            let mut bones_3_data = &bones_3_data[..];
            let bones_3 = (0..bone_count).map(|_| Self::decode_bone_3(&mut bones_3_data)).collect::<Option<Vec<_>>>()?;
            // println!("bones_3 {:?}", bones_3);
        }


        let transform_matrix = (0..12).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;

        // println!("{:?}", transform_matrix);

        let mut materials = Vec::with_capacity(material_count as usize);

        for _ in 0..material_count as usize {
            let id = data.grab_u32_le()?;
            let name = data.grab_str_until_nul()?.to_owned();
            let padding = data.grab_u32_le()?;
            let base_texture_id = data.grab_u32_le()?;
            let bumpmap_texture_id = data.grab_u32_le()?;
            let reflect_texture_id = data.grab_u32_le()?;
            let unknown_1 = data.grab_u32_le()?;
            let max_texture_layers = data.grab_u32_le()?;
            let glow_strength = data.grab_u32_le()?;
            let unknown_2 = data.grab_u8()?;
            let alpha_enabled = data.grab_u8()? > 0;
            let unknown_3 = data.grab_u8()?;
            let ignored = data.grab_u16_le()?; // ?

            materials.push(MeshMaterial {
                id,
                name,
                padding,
                base_texture_id,
                bumpmap_texture_id,
                reflect_texture_id,
                unknown_1,
                max_texture_layers,
                glow_strength,
                unknown_2,
                alpha_enabled,
                unknown_3,
                ignored,
            });
        }

        // println!("materials {:#?}", materials);

        let mut sub_meshes = Vec::new();

        {
        // for i in 0..surface_count {
            // for i in 0..64 {
            //     println!("attempting face_bone_indices size {:?}", i);
            //     let mut data = data.clone();
                // println!("{:02X?}", &data[..32]);

                let hierarchy = data.grab_u32_le()?;
                let destroyable_mesh_levels = data.grab_u32_le()?;
                let floats_1 = (0..5).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
                // println!("floats_1 {:?}", floats_1);
                let n_face_vertex_indices = data.grab_u32_le()?;
                // println!("n_face_vertex_indices {:?}", n_face_vertex_indices);
                let n_face_vertex_indices_bone_index = data.grab_u32_le()?;
                // println!("n_face_vertex_indices_bone_index {:?}", n_face_vertex_indices_bone_index);
                let n_verts = data.grab_u32_le()?;
                let n_faces = data.grab_u32_le()?;
                let n_source_verts = data.grab_u32_le()?;
                let unknown_1 = data.grab_u32_le()?;
                let unknown_2 = data.grab_u32_le()?;
                let unknown_3 = data.grab_u32_le()?;
                let face_indices = data.grab(15 * n_face_vertex_indices as usize)?.to_owned();
                // let face_indices = (0..n_face_vertex_indices).map(|_| data, 15).map(|x| x.to_owned())).collect::<Option<Vec<_>ab(_>>()?;
                // println!("face_indices {:?}", face_indices);
                // 28 looks best for MESH_RIVAL_HERO_APPRENTICE_FEMALE and MESH_TROLL_EARTH_01 (3)
                // 22 looks best for MESH_STAG_BEETLE (2)
                let face_bone_indices = (0..n_face_vertex_indices_bone_index).map(|_| Self::decode_face_bone_index(&mut data)).collect::<Option<Vec<_>>>()?;
                // println!("face_bone_indices {:?}", face_bone_indices);
                let floats_2 = (0..8).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
                // println!("floats_2 {:?}", floats_2);
                let s_vert = data.grab_u32_le()?;
                let padding = data.grab_u32_le()?;
                // println!("padding {:?}", padding);

                // if unknown_2 > 1 && unknown_3 == 0 && s_vert > 20 {
                // println!("name {:?}", name);
                // println!("hierarchy {:?}", hierarchy);
                // println!("destroyable_mesh_levels {:?}", destroyable_mesh_levels);
                // println!("n_verts {:?}", n_verts);
                // println!("n_faces {:?}", n_faces);
                // println!("n_source_verts {:?}", n_source_verts);
                // println!("unknown_1 {:?}", unknown_1);
                // println!("unknown_2 {:?}", unknown_2);
                // println!("unknown_3 {:?}", unknown_3);
                // println!("s_vert {:?}", s_vert);
                // }

            //     println!("");
            // }

            // MESH_HERO_ASSASSIN_SHIRT_01
            // MESH_HERO_FOLDED_DEMONIC_HELM_GOOD

            // let vertex_buffer = data.grab(n_verts as usize * s_vert as usize)?.to_owned();
            let vertex_data_compressed_len = data.grab_u16_le()?;

            let vertex_data_compressed_len = if vertex_data_compressed_len == 0xFFFF {
                data.grab_u32_le()? as usize
            } else {
                vertex_data_compressed_len as usize
            };

            // println!("vertex_data_compressed_len {:?}", vertex_data_compressed_len);

            let vertex_data = Self::decode_semi_compressed(
                &mut data,
                vertex_data_compressed_len as usize,
                n_verts as usize * s_vert as usize * (destroyable_mesh_levels as usize).max(1)
            )?;

            let mut vertex_data = &vertex_data[..];

            // println!("vertex_data {} {:02X?}", vertex_data.len(), vertex_data.iter().map(|x| format!("{:02X?}", x)).collect::<Vec<_>>().join(" "));

            let index_data_compressed_len = data.grab_u16_le()?;

            let index_data_compressed_len = if index_data_compressed_len == 0xFFFF {
                data.grab_u32_le()? as usize
            } else {
                index_data_compressed_len as usize
            };

            // let group_size =
            //     if unknown_2 > 0 { unknown_2 as usize }
            //     else if unknown_3 > 0 { unknown_3 as usize * 2 }
            //     else { 0 };

            // println!("index_data_compressed_len {:02X?}", &data[..index_data_compressed_len as usize]);

            let index_data = Self::decode_semi_compressed(
                &mut data,
                index_data_compressed_len as usize,
                2 * n_source_verts as usize * (destroyable_mesh_levels as usize).max(1),
            )?;

            // println!("index_data {} {:02X?}", index_data.len(), index_data.iter().map(|x| format!("{:02X?}", x)).collect::<Vec<_>>().join(" "));

            // println!("{}, {}, {}, {}, {}, {}, {},", name, n_verts, s_vert, unknown_1, unknown_2, unknown_3, (&vertex_data[..64.min(vertex_data.len())]).iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", "));

            {
                for i in 0..n_verts {
                    let vert = vertex_data.grab(s_vert as usize)?.to_owned();
                    let mut vert = &vert[..];

                    match s_vert {
                        36 => {
                            let x1 = vert.grab_f32_le()?;
                            let y1 = vert.grab_f32_le()?;
                            let z1 = vert.grab_f32_le()?;

                            let u = vert.grab_f32_le()?;
                            let v = vert.grab_f32_le()?;

                            // let rest_chunks = rest.chunks(4).collect::<Vec<_>>();

                            // println!("v {:?} {:?} {:?} {:?} {:?} {:02X?}", x1, y1, z1, u, v, vert);
                            // println!("v {:?} {:?} {:?}", x2, y2, z2);

                            println!("v {:?} {:?} {:?}", x1, y1, z1);
                            // println!("v {:?} {:?} {:?} {:?} {:?}", x2, y2, z2, w2, id);
                        },
                        28 => {
                            let x1 = vert.grab_f32_le()?;
                            let y1 = vert.grab_f32_le()?;
                            let z1 = vert.grab_f32_le()?;

                            // let a = vert.grab_u16_le()?;
                            // let a = vert.grab_u16_le()?;
                            let u = vert.grab_f32_le()?;
                            let v = vert.grab_f32_le()?;

                            // let f1 = vert.grab_f32_le()?;
                            // let f2 = vert.grab_f32_le()?;

                            println!("v {:?} {:?} {:?}", x1, y1, z1);
                            // println!("v {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:02X?}", x1, y1, z1, a, u, v, f1, vert);
                        },
                        20 => {
                            // let _ = vert.grab_u16_le()?;
                            // let _ = vert.grab_u16_le()?;
                            // let _ = vert.grab_u16_le()?;
                            // let _ = vert.grab_u16_le()?;
                            // let x1 = vert.grab_f32_le()?;

                            // let y1 = vert.grab_f32_le()?;

                            // let z1 = vert.grab_f32_le()?;

                            // println!("v {:?} {:?} {:?}", x1, y1, z1);
                        }
                        _ => {

                        }
                    }
                    // let v = Self::decode_vertex_2(&mut vertex_data)?;
                    // println!("v{:?} {:?}", i, v);
                }

                let mut indices = &index_data[..];

                // let _ = indices.grab(16);

                for i in 0 .. (n_source_verts) / 3 {
                    // println!("{:?}, {:?}, {:02X?}", n_source_verts, i, indices.get(..12));
                    let v1 = indices.grab_u16_le()?;
                    let v2 = indices.grab_u16_le()?;
                    let v3 = indices.grab_u16_le()?;
                    // let unknown = indices.grab(3);
                    println!("f {:?} {:?} {:?}", v1 + 1, v2 + 1, v3 + 1);
                    // println!();
                }
            }

            // {
            //     let mut index_data = &index_data[..];

            //     let mut indices = Vec::new();

            //     while let Some(x) = index_data.grab_u16_le() {
            //         indices.push(x);
            //     }

            //     println!("{:?}, {}, {}, {}, {}, {}, {},", name, n_source_verts, s_vert, unknown_1, unknown_2, unknown_3, (&indices[..48.min(indices.len())]).iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", "))
            // }

            // println!("{:02X?}", &data[..32]);

            // println!("{:?}, {}, {}, {},", name, n_verts, s_vert, (&vertex_data[..48]).iter().map(|x|
            // format!("{}", x)).collect::<Vec<String>>().join(", "));

            sub_meshes.push(MeshSubMesh {
                hierarchy,
                destroyable_mesh_levels,
                floats_1,
                n_face_vertex_indices,
                n_face_vertex_indices_bone_index,
                n_verts,
                n_faces,
                n_source_verts,
                unknown_1,
                unknown_2,
                unknown_3,
                face_indices,
                face_bone_indices,
                floats_2,
                s_vert,
                padding,
                // vertex_buffer,
            });
        }

        // println!("{:#?}", sub_meshes);

        Some(Mesh {
            name,
            has_skeleton,
            model_origin,
            helper_points_count,
            helper_dummies_count,
            helper_index_uncompressed,
            padding,
            helper_points,
            helper_dummies,
            material_count,
            surface_count,
            bone_count,
            bone_index_size,
            unknown3,
            unknown4,
            unknown5,
        })
    }

    pub fn decode_semi_compressed(data: &mut &[u8], compressed: usize, mut size: usize) -> Option<Vec<u8>> {
        // println!("compression output len {:?}", size);

        let mut uncompressed_data = Vec::with_capacity(size);

        if compressed > 0 {
            let input = data.grab(compressed as usize)?;
            let out = crate::lzo::decompress(&input, size);
            let out = match out {
                Err(e) => {
                    // println!("decompress error {:?}", e);
                    return None
                },
                Ok(r) => r,
            };

            size -= out.len() as usize;

            // println!("decompression output buffer unused {:?}", size);

            uncompressed_data.extend(&out);
        }

        if size > 0 {
            uncompressed_data.extend(data.grab(size)?);
        }

        Some(uncompressed_data)
    }

    fn decode_bone_1(data: &mut &[u8]) -> Option<MeshBone1> {
        let index = data.grab_i32_le()?;
        let parent = data.grab_i32_le()?;
        let child_count = data.grab_i32_le()?;
        let matrix = (0..12).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
        Some(MeshBone1 {
            index,
            parent,
            child_count,
            matrix,
        })
    }

    fn decode_bone_2(data: &mut &[u8]) -> Option<MeshBone2> {
        let matrix = (0..12).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
        Some(MeshBone2 {
            matrix,
        })
    }

    fn decode_bone_3(data: &mut &[u8]) -> Option<MeshBone3> {
        let matrix = (0..16).map(|_| data.grab_f32_le()).collect::<Option<Vec<_>>>()?;
        Some(MeshBone3 {
            matrix,
        })
    }

    fn decode_face_bone_index(data: &mut &[u8]) -> Option<MeshFaceBoneIndex> {
        let part_1 = data.grab(18)?.to_owned();
        let part_2_length = data.grab_u8()?;
        let part_2 = data.grab(part_2_length as usize)?.to_owned();
        Some(MeshFaceBoneIndex {
            part_1,
            part_2_length,
            part_2,
        })
    }

    fn decode_vertex_1(data: &mut &[u8]) -> Option<MeshVertex1> {
        let unknown_1 = data.grab_f32_le()?;
        let unknown_2 = data.grab_f32_le()?;
        let unknown_3 = data.grab_u16_le()?;
        let unknown_4 = data.grab_u16_le()?;
        Some(MeshVertex1 {
            unknown_1,
            unknown_2,
            unknown_3,
            unknown_4,
        })
    }

    fn decode_vertex_2(data: &mut &[u8]) -> Option<MeshVertex2> {
        let unknown = data.grab(20)?.to_owned();

        // let unknown_1 = data.grab_f32_le()?;
        // let unknown_2 = data.grab_f32_le()?;
        // let unknown_3 = data.grab_u16_le()?;
        // let unknown_4 = data.grab_u16_le()?;
        Some(MeshVertex2 {
            unknown
        })
    }
}