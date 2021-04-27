use std::io::{Read,Seek};

use crate::{Bytes,BadPos};

#[derive(Debug)]
pub struct StbLev {
    pub offset: u32,
    pub compressed_size: u32,
    pub start_x: f32,
    pub start_y: f32,
    pub start_z: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub end_z: f32,
    pub unknown_1: u32,
}

impl StbLev {
    pub fn decode<T: Read + Seek>(mut source: T, _block_count: usize) -> Result<StbLev, BadPos> {
        let mut data = Vec::new();

        source.read_to_end(&mut data).or(Err(BadPos))?;

        let mut data = &data[..];

        let first_block = Bytes::take(&mut data, 2048)?;

        println!("{:?}", first_block);

        let second_block_len = data.take_u32_le()? as usize;
        let second_block = Bytes::take(&mut data, second_block_len + (second_block_len % 2048))?.to_owned();
        let second_block = (&second_block[..second_block_len]).to_owned();

        // let second_block = Bytes::take(&mut data, (second_block_len as usize).min(2048) - 4)?;

        println!("{:?}", second_block);

        // println!("second block len {:?}", second_block_len);

        let mut blocks = Vec::new();

        while blocks.len() < 19 {
            let decompressed_size = data.take_u32_le()?;
            let compressed_len = data.take_u32_le()?;
            println!("{:?} {:?}", decompressed_size, compressed_len);
            let compressed_data = Bytes::take(&mut data, compressed_len as usize)?.to_owned();
            Bytes::take(&mut data, 2040usize.saturating_sub(compressed_data.len()))?;
            // println!("{:?} {:?} {:x?}", decompressed_size, compressed_len, compressed_data);
            let decompressed = crate::lzo::decompress(&compressed_data, decompressed_size as usize);
            // println!("{:?} {:?} {:?}", decompressed_size, compressed_len, decompressed);
            // println!("{:?} {:?} {:?}", decompressed_size, compressed_len, decompressed);
            blocks.push((decompressed_size, compressed_len, compressed_data));
        }

        // source.seek(SeekFrom::Start(2048)).or(Err(BadPos))?;
        // source.read_to_end(&mut data).or(Err(BadPos))?;

        // let mut data = &data[..];

        // let offset = data.take_u32_le()?;
        // let compressed_size = data.take_u32_le()?;
        // let start_x = data.take_f32_le()?;
        // let start_y = data.take_f32_le()?;
        // let start_z = data.take_f32_le()?;
        // let end_x = data.take_f32_le()?;
        // let end_y = data.take_f32_le()?;
        // let end_z = data.take_f32_le()?;
        // let unknown_1 = data.take_u32_le()?;

        // println!("offset {:?}", offset);
        // println!("compressed_size {:?}", compressed_size);
        // println!("start_x {:?}", start_x);
        // println!("start_y {:?}", start_y);
        // println!("start_z {:?}", start_z);
        // println!("end_x {:?}", end_x);
        // println!("end_y {:?}", end_y);
        // println!("end_z {:?}", end_z);
        // println!("unknown_1 {:?}", unknown_1);

        // let compressed_data = &original[offset as usize .. offset as usize + compressed_size as usize];

        // println!("{} {:x?}", compressed_data.len(), compressed_data);

        // println!("compressed_data {:?}", compressed_data);

        // let decompressed_data = crate::lzo::decompress(&compressed_data, 2048);

        // println!("{:?}", decompressed);

        // let lzo_ctx = rust_lzo::LZOContext::new();

        // let mut decompressed_data = Vec::with_capacity(4096);

        // rust_lzo::LZOContext::decompress_to_slice(compressed_data.clone(), &mut decompressed_data[..]);

        // println!("{:?}", decompressed_data);

        Err(BadPos)
    }
}