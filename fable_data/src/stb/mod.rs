mod stb_lev;

pub use stb_lev::*;

use std::io::{Read,Seek,SeekFrom,Cursor};
use std::collections::HashMap;

use crate::{Bytes};

#[derive(Debug)]
pub struct Stb {
    pub magic_number: String,
    pub version: (u32, u32, u32),
    pub block_size: u32,
    pub entry_count: u32,
    pub level_count: u32,
    pub entries_start: u32,
    pub unknown_1: u32,
    pub unknown_2: u32,
    pub level_count_again: u32,
    pub entries: Vec<StbEntry>,
}

#[derive(Debug)]
pub struct StbEntry {
    pub unknown_1: u32,
    pub id: u32,
    pub unknown_2: u32,
    pub len: u32,
    pub pos: u32,
    pub unknown_3: u32,
    pub name_1: String,
    pub unknown_4: u32,
    pub unknown_5: u32,
    pub name_2: String,
    pub extras_len: u32,
    pub extras: Option<StbEntryExtras>,
}

#[derive(Debug)]
pub struct StbEntryExtras {
    pub unknown_1: u32,
    pub unknown_2: u32,
    pub unknown_3: u32,
    pub unknown_4: u32,
}

pub struct StbStaticMapCommon {
    pub entries_count: u32,
    pub entries: HashMap<String, StbStaticMapEntry>,
}

pub struct StbStaticMapEntry {
    pub offset: u32,
    pub unknown_1: u32,
    pub id: u32,
    pub unknown_2: u32,
    pub width: u32,
    pub height: u32,
}

impl Stb {
    pub fn decode<T: Read + Seek>(mut source: T) -> Result<Self, BadPos> {
        let mut header_data = &mut [0; 32][..];

        source.read_exact(&mut header_data).or(Err(BadPos))?;

        let magic_number = header_data.take_as_str(4)?.to_owned();
        let version = (header_data.take_u32_le()?, header_data.take_u32_le()?, header_data.take_u32_le()?);
        let block_size = header_data.take_u32_le()?;
        let entry_count = header_data.take_u32_le()?;
        let level_count = header_data.take_u32_le()?;
        let entries_start = header_data.take_u32_le()?;

        // println!("magic_number {:?}", magic_number);
        // println!("version {:?}", version);
        // println!("block_size {:?}", block_size);
        // println!("entry_count {:?}", entry_count);
        // println!("level_count {:?}", level_count);
        // println!("entries_start {:?}", entries_start);

        let mut entries_data = Vec::new();

        source.seek(SeekFrom::Start(entries_start as u64)).or(Err(BadPos))?;
        source.read_to_end(&mut entries_data).or(Err(BadPos))?;

        let mut entries_data = &mut entries_data[..];

        let unknown_1 = entries_data.take_u32_le()?;
        let unknown_2 = entries_data.take_u32_le()?;
        let level_count_again = entries_data.take_u32_le()?;

        // println!("unknown_1 {:?}", unknown_1);
        // println!("unknown_2 {:?}", unknown_2);
        // println!("level_count_again {:?}", level_count_again);

        let mut entries = Vec::new();

        while entries.len() < level_count_again as usize {
            let unknown_1 = entries_data.take_u32_le()?;
            let id = entries_data.take_u32_le()?;
            let unknown_2 = entries_data.take_u32_le()?;
            let len = entries_data.take_u32_le()?;
            let pos = entries_data.take_u32_le()?;
            let unknown_3 = entries_data.take_u32_le()?;

            let name_1_len = entries_data.take_u32_le()?;
            let name_1 = entries_data.take_as_str(name_1_len as usize)?.to_owned();

            let unknown_4 = entries_data.take_u32_le()?;
            let unknown_5 = entries_data.take_u32_le()?;

            let name_2_len = entries_data.take_u32_le()?;
            let name_2 = entries_data.take_as_str(name_2_len as usize)?.to_owned();

            let extras_len = entries_data.take_u32_le()?;
            let extras = if extras_len == 16 {
                let unknown_1 = entries_data.take_u32_le()?;
                let unknown_2 = entries_data.take_u32_le()?;
                let unknown_3 = entries_data.take_u32_le()?;
                let unknown_4 = entries_data.take_u32_le()?;
                Some(StbEntryExtras {
                    unknown_1,
                    unknown_2,
                    unknown_3,
                    unknown_4,
                })
            } else {
                None
            };

            entries.push(StbEntry {
                unknown_1,
                id,
                unknown_2,
                len,
                pos,
                unknown_3,
                name_1,
                unknown_4,
                unknown_5,
                name_2,
                extras_len,
                extras,
            });
        }

        Ok(Stb {
            magic_number,
            version,
            block_size,
            entry_count,
            level_count,
            entries_start,
            unknown_1,
            unknown_2,
            level_count_again,
            entries,
        })
    }

    pub fn decode_static_map_common<T: Read + Seek>(&self, mut source: T) -> Result<StbStaticMapCommon, BadPos> {
        let static_map_common_entry = self.entries.iter()
            .find(|x| x.name_1 == "__STATIC_MAP_COMMON_HEADER__")
            .ok_or(BadPos)?;



        let mut data = Vec::new();

        // source.seek(SeekFrom::Start(start as u64)).or(Err(BadPos))?;
        source.read_to_end(&mut data).or(Err(BadPos))?;

        let data_lookback = &data[..];
        let mut data = &data[..];

        let entries_count = data.take_u32_le()?;

        let mut entries = HashMap::with_capacity(entries_count as usize);

        while entries.len() < entries_count as usize {
            let path = data.take_as_str_until_nul()?.to_owned();
            let offset = data.take_u32_le()?;

            let entry_data = &data_lookback[offset as usize..];

            let unknown_1 = data.take_u32_le()?;
            let id = data.take_u32_le()?;
            let unknown_2 = data.take_u32_le()?;
            let width = data.take_u32_le()?;
            let height = data.take_u32_le()?;

            entries.insert(path, StbStaticMapEntry {
                offset,
                unknown_1,
                id,
                unknown_2,
                width,
                height,
            });
        }

        Ok(StbStaticMapCommon {
            entries_count,
            entries,
        })
    }

    pub fn decode_entry<T: Read + Seek>(
        mut source: T,
        static_map_common: &StbStaticMapCommon,
        entry: &StbEntry
    ) -> Result<StbLev, BadPos> {
        let mut data = vec![0; entry.len as usize];

        entry.read_from(&mut source, &mut data)?;

        let static_map_data = static_map_common.entries.get(&entry.name_1).ok_or(BadPos)?;

        let map_block_count = static_map_data.width * static_map_data.height / 256;

        Ok(StbLev::decode(Cursor::new(data), map_block_count as usize)?)
    }
}

impl StbEntry {
    pub fn read_from<T: Read + Seek>(&self, mut source: T, buf: &mut [u8]) -> Result<(), BadPos> {
        let read_buf = buf.get_mut(..self.len as usize).ok_or(BadPos)?;
        source.seek(SeekFrom::Start(self.pos as u64)).or(Err(BadPos))?;
        source.read_exact(read_buf).or(Err(BadPos))?;
        Ok(())
    }
}