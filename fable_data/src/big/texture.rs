

use crate::{BadPos,BigTextureInfo};

#[derive(Debug)]
pub struct Texture {
    // info: BigTextureInfo,
    // data: Vec<u8>,
}

impl Texture {
    pub fn decode(source: &[u8], info: &BigTextureInfo) -> Result<Self, BadPos> {
        // println!("source {:X?}", source);

        // let header = View::take(&mut source, 4)?;

        // println!("header {:X?}", header);

        for i in 0..info.first_mipmap_compressed_size as usize {
            for k in (i..=info.first_mipmap_compressed_size as usize).rev() {
                let first_mipmap_compressed = &source[i..k];



                match crate::lzo::decompress(first_mipmap_compressed, info.first_mipmap_size as usize) {
                    Ok(x) => {
                        println!("{:?} {:?} passed", i, info.first_mipmap_compressed_size as usize)

                        // println!("{:?} {:?}", i, x);

                        // if first_mipmap_compressed.len() > largest {
                        //     largest = first_mipmap_compressed.len();
                        // }
                    },
                    Err(x) => {
                        println!("{:?}..{:?} {:?} error {:?}", i, k, info.first_mipmap_compressed_size as usize, x);
                    }
                }
            }
        }

        // for i in 0..info.first_mipmap_compressed_size as usize {
        //     let first_mipmap_compressed = &source[i..info.first_mipmap_compressed_size as usize];

        //     // println!("block {:X?}", first_mipmap_compressed);

        //     // println!("{:x?}", first_mipmap_compressed);

        //     match crate::lzo::decompress(first_mipmap_compressed, info.first_mipmap_size as usize) {
        //         Ok(x) => {
        //             println!("{:?} {:?} passed", i, info.first_mipmap_compressed_size as usize)

        //             // println!("{:?} {:?}", i, x);

        //             // if first_mipmap_compressed.len() > largest {
        //             //     largest = first_mipmap_compressed.len();
        //             // }
        //         },
        //         Err(x) => {
        //             // println!("{:?} {:?} error {:?}", i, info.first_mipmap_compressed_size as usize, x);
        //         }
        //     }
        // }

        // let data = bcndecode::decode(
        //     &source[..],
        //     info.width as usize,
        //     info.height as usize,
        //     match info.dxt_compression {
        //         1 => bcndecode::BcnEncoding::Bc1,
        //         _ => return Err(BadPos),
        //     },
        //     bcndecode::BcnDecoderFormat::RGBA,
        // ).or(Err(BadPos))?;

        Ok(Texture {
            // info,
            // data,
        })
    }
}