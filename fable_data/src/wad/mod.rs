// use crate::{Bytes,Result,BadPos};

//

mod lev;
mod tng;

pub use lev::*;
pub use tng::*;

use std::io::{Read,Seek,SeekFrom};
use std::collections::HashMap;

use crate::Bytes;

#[derive(Debug)]
pub struct Wad {
    pub magic_number: String,
    pub version: (u32, u32, u32),
    pub block_size: u32,
    pub entry_count: u32,
    pub entry_count_repeat: u32,
    pub entries_start: u32,
    pub entries: Vec<WadEntry>,
}

#[derive(Debug)]
pub struct WadEntry {
    pub unknown_1: Vec<u8>,
    pub id: u32,
    pub unknown_2: u32,
    pub data_size: u32,
    pub data_start: u32,
    pub unknown_3: u32,
    pub path: String,
    pub unknown_4: Vec<u8>,
    pub created: WadTimestamp,
    pub accessed: WadTimestamp,
    pub modified: WadTimestampShort,
}

#[derive(Debug)]
pub struct WadTimestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32
}

#[derive(Debug)]
pub struct WadTimestampShort {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Wad {
    pub fn decode<T: Read + Seek>(mut source: T) -> Option<Self> {
        let mut header = &mut [0; 32][..];

        source.read_exact(&mut header).ok()?;

        let magic_number = header.grab_str(4)?.to_owned();
        let version = (header.grab_u32_le()?, header.grab_u32_le()?, header.grab_u32_le()?);
        let block_size = header.grab_u32_le()?;
        let entry_count = header.grab_u32_le()?;
        let entry_count_repeat = header.grab_u32_le()?;
        let entries_start = header.grab_u32_le()?;

        let mut entries_data = Vec::new();

        source.seek(SeekFrom::Start(entries_start as u64)).ok()?;
        source.read_to_end(&mut entries_data).ok()?;

        let mut entries_data = &entries_data[..];

        let mut entries = Vec::new();

        while entries.len() < entry_count as usize {
            let unknown_1 = entries_data.grab(16)?.to_owned();
            let id = entries_data.grab_u32_le()?;
            let unknown_2 = entries_data.grab_u32_le()?;
            let data_size = entries_data.grab_u32_le()?;
            let data_start = entries_data.grab_u32_le()?;
            let unknown_3 = entries_data.grab_u32_le()?;
            let path = entries_data.grab_str_with_u32_le_prefix()?.to_owned();
            let unknown_4 = entries_data.grab(16)?.to_owned();
            let created = Self::decode_timestamp(&mut entries_data)?;
            let accessed = Self::decode_timestamp(&mut entries_data)?;
            let modified = Self::decode_timestamp_short(&mut entries_data)?;

            entries.push(WadEntry {
                unknown_1,
                id,
                unknown_2,
                data_size,
                data_start,
                unknown_3,
                path,
                unknown_4,
                created,
                accessed,
                modified,
            });
        }

        Some(Wad {
            magic_number,
            version,
            block_size,
            entry_count,
            entry_count_repeat,
            entries_start,
            entries
        })
    }

    fn decode_timestamp(data: &mut &[u8]) -> Option<WadTimestamp> {
        Some(WadTimestamp {
            year: data.grab_u32_le()?,
            month: data.grab_u32_le()?,
            day: data.grab_u32_le()?,
            hour: data.grab_u32_le()?,
            minute: data.grab_u32_le()?,
            second: data.grab_u32_le()?,
            millisecond: data.grab_u32_le()?,
        })
    }

    fn decode_timestamp_short(data: &mut &[u8]) -> Option<WadTimestampShort> {
        Some(WadTimestampShort {
            year: data.grab_u32_le()?,
            month: data.grab_u32_le()?,
            day: data.grab_u32_le()?,
            hour: data.grab_u32_le()?,
            minute: data.grab_u32_le()?,
        })
    }

    pub fn index_by_path(&self) -> HashMap<&String, &WadEntry> {
        let mut index = HashMap::with_capacity(self.entries.len());
        index.extend(self.entries.iter().map(|x| (&x.path, x)));
        index
    }
}

impl WadEntry {
    pub fn read_from<T: Read + Seek>(&self, mut source: T, buf: &mut [u8]) -> Option<()> {
        let read_buf = buf.get_mut(..self.data_size as usize)?;
        source.seek(SeekFrom::Start(self.data_start as u64)).ok()?;
        source.read_exact(read_buf).ok()?;
        Some(())
    }
}