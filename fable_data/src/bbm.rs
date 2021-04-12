use std::io::{Read,Seek};

use views::{Bytes,BadPos};

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
#[derive(Debug,PartialEq)]
pub struct Bbm {
    pub name: String,
    pub has_skeleton: u8,
    pub model_origin: Vec<f32>,
    pub hpnt_count: u16,
    pub hdmy_count: u16,
    pub hlpr_index_uncompressed: u32,
    pub padding: u16,
    pub hpnt_compressed: u16,
    pub helper_points: Vec<BbmHelperPoint>,
    pub hdmy_compressed: u16,
    pub helper_dummies: Vec<BbmHelperDummy>,
    // pub hlpr_index_compressed: u16,
    pub hpnt_index_size: u16,
    pub hpnt_index: Vec<u8>,
    // pub hdmy_index: Vec<u8>,
    // pub material_count: u32,
    // pub surface_count: u32,
    // pub bone_count: u32,
    // pub bone_index_size: u32,
    // pub unknown3: u8,
    // pub unknown4: u16,
    // pub unknown5: u16,
    // pub compressed: u16,
    // pub bone_index_reference: Vec<u16>,
    // pub bone_index_compressed: u16,
    // pub bone_index: Vec<u8>,
    // pub compressed_size: u16,
}

#[derive(Debug,PartialEq)]
pub struct BbmHelperPoint {
    pub matrix: Vec<f32>, // 2x2 matrix?
    pub hierarchy: i32,
}

#[derive(Debug,PartialEq)]
pub struct BbmHelperDummy {
    pub matrix: Vec<f32>, // 13 float matrix?
    pub hierarchy: i32,
}

impl Bbm {
    pub fn decode<T: Read + Seek>(source: &mut T) -> Result<Bbm, BadPos> {
        let mut data = Vec::new();

        source.read_to_end(&mut data).or(Err(BadPos))?;

        let mut data = &data[..];

        let name = data.take_as_str_until_nul()?.to_owned();
        let has_skeleton = data.take_u8()? > 0;
        let model_origin = (0..10).map(|_| data.take_f32_le()).collect::<Result<Vec<_>, _>>()?;
        let hpnt_count = data.take_u16_le()?;
        let hdmy_count = data.take_u16_le()?;
        let hlpr_index_uncompressed = data.take_u32_le()?;
        let padding = data.take_u16_le()?;
        let hpnt_compressed = data.take_u16_le()?;

        println!("name {:?}", name);
        println!("has_skeleton {:?}", has_skeleton);
        println!("model_origin {:?}", model_origin);
        println!("hpnt_count {:?}", hpnt_count);
        println!("hdmy_count {:?}", hdmy_count);
        println!("hlpr_index_uncompressed {:?}", hlpr_index_uncompressed);
        println!("padding {:?}", padding);
        println!("hpnt_compressed {:?}", hpnt_compressed);

        let helper_points = (0..hpnt_count)
            .map(|_| Self::decode_helper_point(&mut data))
            .collect::<Result<Vec<_>, _>>()?;

        println!("helper_points {:?}", helper_points);

        let hdmy_compressed = data.take_u16_le()?;

        let helper_dummies = (0..hdmy_count)
            .map(|_| Self::decode_helper_dummy(&mut data))
            .collect::<Result<Vec<_>, _>>()?;

        println!("hdmy_compressed {:?}", hdmy_compressed);
        println!("helper_dummies {:?}", helper_dummies);

        let hlpr_index_compressed = data.take_u16_le()?;
        let hpnt_index_size = data.take_u16_le()?;

        let hpnt_index = (0..hpnt_index_size.saturating_sub(2))
            .map(|_| data.take_u8())
            .collect::<Result<Vec<_>, _>>()?;

        let hdmy_index = (0..hlpr_index_uncompressed.saturating_sub(hpnt_index_size as u32))
            .map(|_| data.take_u8())
            .collect::<Result<Vec<_>, _>>()?;

        println!("hlpr_index_compressed {:?}", hlpr_index_compressed);
        println!("hpnt_index_size {:?}", hpnt_index_size);
        println!("hpnt_index {:?}", hpnt_index);
        println!("hdmy_index {:?}", hdmy_index);

        let material_count = data.take_u32_le()?;
        let surface_count = data.take_u32_le()?;
        let bone_count = data.take_u32_le()?;
        let bone_index_size = data.take_u32_le()?;
        let unknown3 = data.take_u8()?;
        let unknown4 = data.take_u16_le()?;
        let unknown5 = data.take_u16_le()?;
        let compressed = data.take_u16_le()?;

        let bone_index_reference = (0..bone_count.saturating_sub(1))
            .map(|_| data.take_u16_le())
            .collect::<Result<Vec<_>, _>>()?;

        let bone_index_compressed = data.take_u16_le()?;

        let bone_index_reference = (0..bone_index_size).map(|_| data.take_u8()).collect::<Result<Vec<_>, _>>()?;

        let compressed_size = data.take_u16_le()?;

        println!("material_count {:?}", material_count);
        println!("surface_count {:?}", surface_count);
        println!("bone_count {:?}", bone_count);
        println!("bone_index_size {:?}", bone_index_size);
        println!("unknown3 {:?}", unknown3);
        println!("unknown4 {:?}", unknown4);
        println!("unknown5 {:?}", unknown5);
        println!("compressed {:?}", compressed);
        println!("bone_index_reference {:?}", bone_index_reference);
        println!("bone_index_compressed {:?}", bone_index_compressed);
        println!("bone_index_reference {:?}", bone_index_reference);
        println!("compressed_size {:?}", compressed_size);

        Err(BadPos)
    }

    pub fn decode_helper_point(data: &mut &[u8]) -> Result<BbmHelperPoint, BadPos> {
        let matrix = (0..4).map(|_| data.take_f32_le()).collect::<Result<Vec<_>, _>>()?;
        let hierarchy = data.take_i32_le()?;
        Ok(BbmHelperPoint { matrix, hierarchy })
    }

    pub fn decode_helper_dummy(data: &mut &[u8]) -> Result<BbmHelperPoint, BadPos> {
        let matrix = (0..13).map(|_| data.take_f32_le()).collect::<Result<Vec<_>, _>>()?;
        let hierarchy = data.take_i32_le()?;
        Ok(BbmHelperPoint { matrix, hierarchy })
    }

    // pub fn decode_bbm(input: &[u8]) -> IResult<&[u8], Bbm, Error> {
    //     let (input, header) = Self::decode_header(input)?;

    //     Ok(
    //         (
    //             input,
    //             Bbm {
    //                 name,
    //                 has_skeleton,
    //                 model_origin,
    //                 hpnt_count,
    //                 hdmy_count,
    //                 hlpr_index_uncompressed,
    //                 padding,
    //                 hpnt_compressed,
    //                 helper_points,
    //                 hdmy_compressed,
    //                 helper_dummies,
    //                 // hlpr_index_compressed,
    //                 hpnt_index_size,
    //                 hpnt_index,
    //                 // hdmy_index,
    //                 // material_count,
    //                 // surface_count,
    //                 // bone_count,
    //                 // bone_index_size,
    //                 // unknown3,
    //                 // unknown4,
    //                 // unknown5,
    //                 // compressed,
    //                 // bone_index_reference,
    //                 // bone_index_compressed,
    //                 // bone_index,
    //                 // compressed_size,
    //             }
    //         )
    //     )
    // }

    // pub fn decode_header(input: &[u8]) -> IResult<&[u8], BbmHeader, Error> {
    //     let (input, name) = decode_null_terminated_string(input)?;
    //     let (input, has_skeleton) = le_u8(input)?;
    //     let (input, model_origin) = count(le_f32, 10usize)(input)?;
    //     let (input, hpnt_count) = le_u16(input)?;
    //     let (input, hdmy_count) = le_u16(input)?;
    //     let (input, hlpr_index_uncompressed) = le_u32(input)?;
    //     let (input, padding) = le_u16(input)?;
    //     let (input, hpnt_compressed) = le_u16(input)?;
    //     let (input, helper_points) = count(Self::decode_helper_point, hpnt_count as usize)(input)?;
    //     let (input, hdmy_compressed) = le_u16(input)?;
    //     let (input, helper_dummies) = count(Self::decode_helper_dummy, hdmy_count as usize)(input)?;
    //     // let (input, hlpr_index_compressed) = le_u16(input)?;
    //     let (input, hpnt_index_size) = le_u16(input)?;
    //     let (input, hpnt_index) = count(le_u8, hpnt_index_size.saturating_sub(2) as usize)(input)?;
    //     // let (input, hdmy_index) = count(le_u8, (hlpr_index_uncompressed.checked_sub(hpnt_index_size.into()).unwrap_or(0)) as usize)(input)?;
    //     // let (input, material_count) = le_u32(input)?;
    //     // let (input, surface_count) = le_u32(input)?;
    //     // let (input, bone_count) = le_u32(input)?;
    //     // let (input, bone_index_size) = le_u32(input)?;
    //     // let (input, unknown3) = le_u8(input)?;
    //     // let (input, unknown4) = le_u16(input)?;
    //     // let (input, unknown5) = le_u16(input)?;
    //     // let (input, compressed) = le_u16(input)?;
    //     // let (input, bone_index_reference) = count(le_u16, (bone_count - 1) as usize)(input)?;
    //     // let (input, bone_index_compressed) = le_u16(input)?;
    //     // let (input, bone_index) = count(le_u8, bone_index_size as usize)(input)?;
    //     // let (input, compressed_size) = le_u16(input)?;

    //     dbg!(&name);
    //     dbg!(&has_skeleton);
    //     dbg!(&model_origin);
    //     dbg!(&hpnt_count);
    //     dbg!(&hdmy_count);
    //     dbg!(&hlpr_index_uncompressed);
    //     dbg!(&padding);
    //     dbg!(&hpnt_compressed);
    //     dbg!(&helper_points);
    //     dbg!(&hdmy_compressed);
    //     dbg!(&helper_dummies);
    //     // dbg!(&hlpr_index_compressed);
    //     dbg!(&hpnt_index_size);
    //     dbg!(hpnt_index.len()); // dbg!(&hpnt_index);
    //     // dbg!(&hdmy_index);
    //     // dbg!(&material_count);
    //     // dbg!(&surface_count);
    //     // dbg!(&bone_count);
    //     // dbg!(&bone_index_size);
    //     // dbg!(&unknown3);
    //     // dbg!(&unknown4);
    //     // dbg!(&unknown5);
    //     // dbg!(&compressed);

    //     // hex_table::HexTable::default().format(&input[..256], &mut std::io::stdout()).unwrap();
    //     dbg!(input.len());
    //     println!("");

    //     Ok(
    //         (
    //             input,
    //             BbmHeader {

    //             }
    //         )
    //     )
    // }

    // pub fn decode_helper_point(input: &[u8]) -> IResult<&[u8], BbmHelperPoint, Error> {
    //     let (input, matrix) = count(le_f32, 4usize)(input)?;
    //     let (input, hierarchy) = le_i32(input)?;

    //     Ok(
    //         (
    //             input,
    //             BbmHelperPoint {
    //                 matrix: matrix,
    //                 hierarchy: hierarchy,
    //             }
    //         )
    //     )
    // }

    // pub fn decode_helper_dummy(input: &[u8]) -> IResult<&[u8], BbmHelperDummy, Error> {
    //     let (input, matrix) = count(le_f32, 13usize)(input)?;
    //     let (input, hierarchy) = le_i32(input)?;

    //     Ok(
    //         (
    //             input,
    //             BbmHelperDummy {
    //                 matrix: matrix,
    //                 hierarchy: hierarchy,
    //             }
    //         )
    //     )
    // }
}