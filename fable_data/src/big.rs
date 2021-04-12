use std::io::{Read,Seek,SeekFrom};
use std::collections::HashMap;

use views::{View,Bytes,BadPos};

use crate::BytesExt;

#[derive(Debug)]
pub struct Big {
    pub version: u32,
    pub banks_start: u32,
    pub banks: Vec<BigBank>,

}

#[derive(Debug)]
pub struct BigBank {
    pub name: String,
    pub unknown_1: u32,
    pub entries_count: u32,
    pub index_start: u32,
    pub index_size: u32,
    pub block_size: u32,
    pub file_type_counts: HashMap<u32, u32>,
    pub entries: Vec<BigEntry>,
}

#[derive(Debug)]
pub struct BigEntry {
    pub unknown_1: u32,
    pub id: u32,
    pub kind: u32,
    pub data_size: u32,
    pub data_start: u32,
    pub unknown_2: u32,
    pub name: String,
    pub crc: u32,
    pub sources: Vec<String>,
    // TODO: Figure this out
    pub subheader_size: u32,
    pub subheader: Vec<u8>,
}

impl Big {
    pub fn decode<T: Read + Seek>(mut source: T) -> Result<Self, BadPos> {
        let mut header = &mut [0; 12][..];

        source.read_exact(&mut header).map_err(|_| BadPos)?;

        let _magic_number = header.take_as_str(4)?;
        // if header.take_as_str(4)? != "BIGB" { return Err(BadPos) };

        let version = header.take_u32_le()?;
        let banks_start = header.take_u32_le()?;

        let mut banks_source = vec![];

        source.seek(SeekFrom::Start(banks_start as u64)).or(Err(BadPos))?;
        source.read_to_end(&mut banks_source).or(Err(BadPos))?;

        let mut banks_source = &banks_source[..];
        let banks_count = banks_source.take_u32_le()?;
        let mut banks = Vec::new();

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

            let mut entries = Vec::new();

            while entries.len() < entries_count as usize {
                let unknown_1 = index_source.take_u32_le()?;
                let id = index_source.take_u32_le()?;
                let kind = index_source.take_u32_le()?;
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

                let subheader_size = index_source.take_u32_le()?;
                let subheader = View::take(&mut index_source, subheader_size as usize)?.to_owned();

                entries.push(BigEntry {
                    unknown_1,
                    id,
                    kind,
                    data_size,
                    data_start,
                    unknown_2,
                    name,
                    crc,
                    sources,
                    subheader_size,
                    subheader,
                });
            }

            banks.push(BigBank {
                name,
                unknown_1,
                entries_count,
                index_start,
                index_size,
                block_size,
                file_type_counts,
                entries,
            });
        }

        Ok(Big {
            version,
            banks_start,
            banks,
        })
    }
}

impl BigEntry {
    pub fn read_from<T: Read + Seek>(&self, source: &mut T, buf: &mut [u8]) -> Result<(), BadPos> {
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
            fable_dir.join("data/lang/English/text.big"),
            fable_dir.join("data/lang/English/dialogue.big"),
            fable_dir.join("data/lang/English/fonts.big"),
            fable_dir.join("data/graphics/graphics.big"),
            fable_dir.join("data/graphics/pc/textures.big"),
            fable_dir.join("data/graphics/pc/frontend.big"),
            fable_dir.join("data/shaders/pc/shaders.big"),
            fable_dir.join("data/misc/pc/effects.big"),
        ];

        for path in paths.iter() {
            let mut types:  HashSet<u32> = HashSet::new();

            // println!("{:?}", path);


            let mut file = BufReader::new(File::open(path).unwrap());
            let big = Big::decode(&mut file).unwrap();

            for bank in big.banks.iter() {
                println!("{:?} {:?}\n{:#?}\n", bank.name, path.file_name().unwrap(), bank.file_type_counts);

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