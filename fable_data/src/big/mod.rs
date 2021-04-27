mod anim;
mod mesh;
mod text;
mod texture;

pub use anim::*;
pub use mesh::*;
pub use text::*;
pub use texture::*;

use std::io::{Read,Seek,SeekFrom};
use std::collections::HashMap;
use std::path::Path;

use crate::{View,Bytes,BadPos};

#[derive(Debug)]
pub struct Big {
    pub kind: BigKind,
    pub magic_number: String,
    pub version: u32,
    pub banks_start: u32,
    pub banks: HashMap<String, BigBank>,
}

#[derive(Debug)]
pub struct BigBank {
    pub unknown_1: u32,
    pub entries_count: u32,
    pub index_start: u32,
    pub index_size: u32,
    pub block_size: u32,
    pub file_type_counts: HashMap<u32, u32>,
    pub entries: HashMap<String, BigEntry>,
}

#[derive(Debug)]
pub struct BigEntry {
    pub unknown_1: u32,
    pub id: u32,
    pub group: u32,
    pub data_size: u32,
    pub data_start: u32,
    pub unknown_2: u32,
    pub crc: u32,
    pub sources: Vec<String>,
    pub info: BigInfo,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum BigKind {
    Text,
    Dialogue,
    Fonts,
    Graphics,
    Textures,
    Frontend,
    Shaders,
    Effects,
    Fmp,
    Unknown,
}

#[derive(Debug)]
pub enum BigInfo {
    Mesh(BigMeshInfo),
    Texture(BigTextureInfo),
    Animation(BigAnimationInfo),
    Unknown(Vec<u8>),
}

#[derive(Debug)]
pub struct BigTextureInfo {
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub frame_width: u16,
    pub frame_height: u16,
    pub frame_count: u16,
    pub dxt_compression: u16,
    pub unknown_1: u16,
    pub alpha_channel_count: u8,
    pub mipmaps: u8,
    pub unknown_2: u16,
    pub first_mipmap_size: u32,
    pub first_mipmap_compressed_size: u32,
    pub unknown_3: u16,
    pub unknown_4: u32,
}

#[derive(Debug)]
pub struct BigMeshInfo {
    pub physics_mesh: u32,
    pub unknown_1: Vec<f32>, // 10 floats that are also found in the mesh.
    pub compressed_lod_sizes: Vec<u32>, // length prefixed list
    // pub unknown_2: u32,
    pub texture_ids: Vec<u32>, // length prefixed list
}

#[derive(Debug)]
pub struct BigAnimationInfo {
    unknown: Vec<u8>,
}

impl Big {
    pub fn decode_reader_with_path<T: Read + Seek, P: AsRef<Path>>(source: T, path: P) -> Result<Self, BadPos> {
        let path = path.as_ref();

        let file_name = path.file_name().map(|x| x.to_str().map(|x| x.to_lowercase())).flatten();

        let kind = if path.extension().map(|x| x.to_str()) == Some(Some(".fmp")) {
            BigKind::Fmp
        } else {
            match file_name.as_deref() {
                Some("text.big") => BigKind::Text,
                Some("dialogue.big") => BigKind::Dialogue,
                Some("fonts.big") => BigKind::Fonts,
                Some("graphics.big") => BigKind::Graphics,
                Some("textures.big") => BigKind::Textures,
                Some("frontend.big") => BigKind::Frontend,
                Some("shaders.big") => BigKind::Shaders,
                Some("effects.big") => BigKind::Effects,
                _ => BigKind::Unknown,
            }
        };

        Self::decode_reader(source, kind)
    }

    pub fn decode_reader<T: Read + Seek>(mut source: T, kind: BigKind) -> Result<Self, BadPos> {
        let mut header = &mut [0; 12][..];

        source.read_exact(&mut header).map_err(|_| BadPos)?;

        let magic_number = header.take_as_str(4)?.to_owned();

        let version = header.take_u32_le()?;
        let banks_start = header.take_u32_le()?;

        let banks = Self::decode_banks(source, banks_start, kind)?;

        Ok(Big {
            kind,
            magic_number,
            version,
            banks_start,
            banks,
        })
    }

    fn decode_banks<T: Read + Seek>(mut source: T, banks_start: u32, kind: BigKind) -> Result<HashMap<String, BigBank>, BadPos> {
        let mut banks_source = Vec::new();

        source.seek(SeekFrom::Start(banks_start as u64)).or(Err(BadPos))?;
        source.read_to_end(&mut banks_source).or(Err(BadPos))?;

        let mut banks_source = &banks_source[..];
        let banks_count = banks_source.take_u32_le()?;
        let mut banks = HashMap::new();

        while banks.len() < banks_count as usize {
            let name = std::str::from_utf8(banks_source.take_until_nul()?).map_err(|_| BadPos)?.to_owned();
            let unknown_1 = banks_source.take_u32_le()?;
            let entries_count = banks_source.take_u32_le()?;
            let index_start = banks_source.take_u32_le()?;
            let index_size = banks_source.take_u32_le()?;
            let block_size = banks_source.take_u32_le()?;

            let mut index_source = &mut vec![0; index_size as usize][..];

            source.seek(SeekFrom::Start(index_start as u64)).or(Err(BadPos))?;
            source.read_exact(&mut index_source).or(Err(BadPos))?;

            let file_types_count = index_source.take_u32_le()?;
            let mut file_type_counts = HashMap::new();

            while file_type_counts.len() < file_types_count as usize {
                let a = index_source.take_u32_le()?;
                let b = index_source.take_u32_le()?;
                file_type_counts.insert(a, b);
            }

            let entries = Self::decode_entries(index_source, entries_count, kind)?;

            banks.insert(name, BigBank {
                unknown_1,
                entries_count,
                index_start,
                index_size,
                block_size,
                file_type_counts,
                entries,
            });
        }

        Ok(banks)
    }

    fn decode_entries(mut index_source: &[u8], entries_count: u32, kind: BigKind) -> Result<HashMap<String, BigEntry>, BadPos> {
        let mut entries = HashMap::new();

        while entries.len() < entries_count as usize {
            let unknown_1 = index_source.take_u32_le()?;
            let id = index_source.take_u32_le()?;
            let group = index_source.take_u32_le()?;
            let data_size = index_source.take_u32_le()?;
            let data_start = index_source.take_u32_le()?;
            let unknown_2 = index_source.take_u32_le()?;
            let name = index_source.take_as_str_with_u32_le_prefix()?.to_owned();
            let crc = index_source.take_u32_le()?;

            let sources_count = index_source.take_u32_le()?;
            let mut sources = Vec::new();

            while sources.len() < sources_count as usize {
                sources.push(index_source.take_as_str_with_u32_le_prefix()?.to_owned());
            }

            let info_size = index_source.take_u32_le()?;
            let info_data = View::take(&mut index_source, info_size as usize)?;
            let info = match (kind, group) {
                // (BigKind::Graphics, 0) => BigInfo::Unknown,
                // Normal mesh?
                (BigKind::Graphics, 1) => BigInfo::Mesh(Self::decode_mesh_info(info_data)?),
                // Flora mesh?
                (BigKind::Graphics, 2) => BigInfo::Mesh(Self::decode_mesh_info(info_data)?),
                // Physics mesh
                (BigKind::Graphics, 3) => BigInfo::Mesh(Self::decode_mesh_info(info_data)?),
                (BigKind::Graphics, 4) => BigInfo::Mesh(Self::decode_mesh_info(info_data)?),
                (BigKind::Graphics, 5) => BigInfo::Mesh(Self::decode_mesh_info(info_data)?),
                // Normal animation?
                (BigKind::Graphics, 6) => BigInfo::Animation(Self::decode_animation_info(info_data)?),
                (BigKind::Graphics, 7) => BigInfo::Animation(Self::decode_animation_info(info_data)?),
                // (BigKind::Graphics, 8) => BigInfo::Unknown,
                (BigKind::Graphics, 9) => BigInfo::Animation(Self::decode_animation_info(info_data)?),

                (BigKind::Textures, 0) => BigInfo::Texture(Self::decode_texture_info(info_data)?),
                (BigKind::Textures, 1) => BigInfo::Texture(Self::decode_texture_info(info_data)?),
                // Bump maps
                (BigKind::Textures, 2) => BigInfo::Texture(Self::decode_texture_info(info_data)?),
                // (BigKind::Textures, 3) => BigInfo::Unknown,
                (BigKind::Textures, 3) => BigInfo::Texture(Self::decode_texture_info(info_data)?),
                (BigKind::Textures, 4) => BigInfo::Texture(Self::decode_texture_info(info_data)?),
                (BigKind::Textures, 5) => BigInfo::Texture(Self::decode_texture_info(info_data)?),

                // (BigKind::Text, 0) => BigInfo::Unknown,

                (_, _) => BigInfo::Unknown(info_data.to_owned())
            };

            entries.insert(name, BigEntry {
                unknown_1,
                id,
                group,
                data_size,
                data_start,
                unknown_2,
                crc,
                sources,
                info,
            });
        }

        Ok(entries)
    }

    // fn decode_entry_kind(big_kind, group_id) -> BigEntryKind {
    //     match (big_kind) {
    //         ()
    //         _ => BigEntryKind::Unknown,
    //     }
    // }

    fn decode_mesh_info(mut info_data: &[u8]) -> Result<BigMeshInfo, BadPos> {
        let physics_mesh = info_data.take_u32_le()?;

        // println!("phyiscs_mesh {:?}", physics_mesh);

        let mut unknown_1 = Vec::new();
        for _ in 0..10 {
            unknown_1.push(info_data.take_f32_le()?)
        }

        // println!("unknown_1 {:?}", unknown_1);

        let compressed_lod_sizes_count = info_data.take_u32_le()?;

        // println!("compressed_lod_sizes_count {:?}", compressed_lod_sizes_count);

        let mut compressed_lod_sizes = Vec::new();
        for _ in 0..compressed_lod_sizes_count as usize {
            compressed_lod_sizes.push(info_data.take_u32_le()?);
        }

        // println!("compressed_lod_sizes {:?}", compressed_lod_sizes);

        // let unknown_2 = info_data.take_u32_le()?;

        // println!("unknown_2 {:?}", unknown_2);

        let texture_ids_count = info_data.take_u32_le()?;

        // println!("texture_ids_count {:?}", texture_ids_count);

        let mut texture_ids = Vec::new();
        for _ in 0..texture_ids_count as usize {
            texture_ids.push(info_data.take_u32_le()?);
        }

        // println!("texture_ids {:?}", texture_ids);

        Ok(BigMeshInfo {
            physics_mesh,
            unknown_1,
            compressed_lod_sizes,
            // unknown_2,
            texture_ids,
        })
    }

    fn decode_texture_info(mut info_data: &[u8]) -> Result<BigTextureInfo, BadPos> {
        let width = info_data.take_u16_le()?;
        let height = info_data.take_u16_le()?;
        let depth = info_data.take_u16_le()?;
        let frame_width = info_data.take_u16_le()?;
        let frame_height = info_data.take_u16_le()?;
        let frame_count = info_data.take_u16_le()?;
        let dxt_compression = info_data.take_u16_le()?;
        let unknown_1 = info_data.take_u16_le()?;
        let alpha_channel_count = info_data.take_u8()?;
        let mipmaps = info_data.take_u8()?;
        let unknown_2 = info_data.take_u16_le()?;
        let first_mipmap_size = info_data.take_u32_le()?;
        let first_mipmap_compressed_size = info_data.take_u32_le()?;
        let unknown_3 = info_data.take_u16_le()?;
        let unknown_4 = info_data.take_u32_le()?;

        Ok(BigTextureInfo {
            width,
            height,
            depth,
            frame_width,
            frame_height,
            frame_count,
            dxt_compression,
            unknown_1,
            alpha_channel_count,
            mipmaps,
            unknown_2,
            first_mipmap_size,
            first_mipmap_compressed_size,
            unknown_3,
            unknown_4,
        })
    }

    fn decode_animation_info(info_data: &[u8]) -> Result<BigAnimationInfo, BadPos> {
        Ok(BigAnimationInfo {
            unknown: info_data.to_vec(),
        })
    }
}

impl BigEntry {
    pub fn read_from<T: Read + Seek>(&self, mut source: T, buf: &mut [u8]) -> Result<(), BadPos> {
        let read_buf = buf.get_mut(..self.data_size as usize).ok_or(BadPos)?;
        source.seek(SeekFrom::Start(self.data_start as u64)).or(Err(BadPos))?;
        source.read_exact(read_buf).or(Err(BadPos))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::collections::{HashMap,HashSet};

    use super::*;

    #[test]
    fn test_big_file_types() {
        let fable_dir = PathBuf::from(env::var("FABLE_DIR").expect("FABLE_DIR env var not given."));

        let paths = [
            // fable_dir.join("data/lang/English/text.big"),
            // fable_dir.join("data/lang/English/dialogue.big"),
            // fable_dir.join("data/lang/English/fonts.big"),
            // fable_dir.join("data/graphics/graphics.big"),
            fable_dir.join("data/graphics/pc/textures.big"),
            // fable_dir.join("data/graphics/pc/frontend.big"),
            // fable_dir.join("data/shaders/pc/shaders.big"),
            // fable_dir.join("data/misc/pc/effects.big"),
        ];

        for path in paths.iter() {
            let mut types:  HashSet<u32> = HashSet::new();

            // println!("{:?}", path);

            let mut file = BufReader::new(File::open(path).unwrap());
            let big = Big::decode_reader_with_path(&mut file, path).unwrap();

            for (bank_name, bank) in big.banks.iter() {
                // println!("{:?} {:?}\n{:#?}\n", bank_name, path.file_name().unwrap(), bank.file_type_counts);

                // for (_id, type_id) in bank.index.file_types.iter() {
                //     match types.get_mut(type_id) {
                //         None => {
                //             let mut x = HashSet::new();
                //             x.insert(path.clone());
                //             types.insert(*type_id, x);
                //         }
                //         Some(set) => {
                //             set.insert(path.clone());
                //         }
                //     }
                // }

                // for entry in bank.entries.iter() {
                //     if !types.contains(&entry.unknown_2) {
                //         let mut x = HashSet::new();
                //         x.insert(path.clone());
                //         println!("{:?}\n{:#?}\n", path, entry);
                //         types.insert(entry.unknown_2);
                //     }
                // }

                // for entry in bank.entries.iter() {
                //     if entry.kind == 0 {
                //         println!("{:#?}", entry);
                //     }
                // }
            }
        }
    }
}