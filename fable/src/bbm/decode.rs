use std::io::{Read,Seek};

use nom::IResult;
use nom::number::complete::{le_u8,le_u16,le_u32,le_i32,le_f32};
use nom::multi::count;
use nom::combinator::all_consuming;

use crate::{Decode,Error};
use crate::shared::decode_null_terminated_string;

use super::{
    Bbm,
    BbmHeader,
    BbmHelperPoint,
    BbmHelperDummy,
};

impl<Source: Read + Seek> Decode<Bbm> for Source {
    fn decode(&mut self) -> Result<Bbm, Error> {
        let mut input = Vec::new();
        self.read_to_end(&mut input)?;
        let (_, bbm) = all_consuming(Bbm::decode_bbm)(&input)?;
        Ok(bbm)
    }
}

impl Bbm {
    pub fn decode_bbm(input: &[u8]) -> IResult<&[u8], Bbm, Error> {
        let (input, header) = Self::decode_header(input)?;

        Ok(
            (
                input,
                Bbm {
                    header: header,
                }
            )
        )
    }

    pub fn decode_header(input: &[u8]) -> IResult<&[u8], BbmHeader, Error> {
        let (input, unknown1) = decode_null_terminated_string(input)?;
        let (input, selection_present) = le_u8(input)?;
        let (input, unknown2) = count(le_f32, 10usize)(input)?;
        let (input, hpnt_count) = le_u16(input)?;
        let (input, hdmy_count) = le_u16(input)?;
        let (input, hlpr_index_uncompressed) = le_u32(input)?;
        let (input, padding) = le_u16(input)?;
        let (input, hpnt_compressed) = le_u16(input)?;
        let (input, helper_points) = count(Self::decode_helper_point, hpnt_count as usize)(input)?;
        let (input, hdmy_compressed) = le_u16(input)?;
        let (input, helper_dummies) = count(Self::decode_helper_dummy, hdmy_count as usize)(input)?;
        let (input, hlpr_index_compressed) = le_u16(input)?;
        let (input, hpnt_index_size) = le_u16(input)?;
        let (input, hpnt_index) = count(le_u8, (hpnt_index_size - 2) as usize)(input)?;
        let (input, hdmy_index) = count(le_u8, (hlpr_index_compressed - hpnt_index_size) as usize)(input)?;
        let (input, material_count) = le_u32(input)?;
        let (input, surface_count) = le_u32(input)?;
        let (input, bone_count) = le_u32(input)?;
        let (input, bone_index_size) = le_u32(input)?;
        let (input, unknown3) = le_u16(input)?;
        let (input, unknown4) = le_u16(input)?;
        let (input, unknown5) = le_u16(input)?;
        let (input, compressed) = le_u16(input)?;
        let (input, bone_index_reference) = count(le_u16, (bone_count - 1) as usize)(input)?;
        let (input, bone_index_compressed) = le_u16(input)?;
        let (input, bone_index) = count(le_u8, bone_index_size as usize)(input)?;
        let (input, compressed_size) = le_u16(input)?;

        Ok(
            (
                input,
                BbmHeader {
                    unknown1: unknown1,
                    selection_present: selection_present,
                    unknown2: unknown2,
                    hpnt_count: hpnt_count,
                    hdmy_count: hdmy_count,
                    hlpr_index_uncompressed: hlpr_index_uncompressed,
                    padding: padding,
                    hpnt_compressed: hpnt_compressed,
                    helper_points: helper_points,
                    hdmy_compressed: hdmy_compressed,
                    helper_dummies: helper_dummies,
                    hlpr_index_compressed: hlpr_index_compressed,
                    hpnt_index_size: hpnt_index_size,
                    hpnt_index: hpnt_index,
                    hdmy_index: hdmy_index,
                    material_count: material_count,
                    surface_count: surface_count,
                    bone_count: bone_count,
                    bone_index_size: bone_index_size,
                    unknown3: unknown3,
                    unknown4: unknown4,
                    unknown5: unknown5,
                    compressed: compressed,
                    bone_index_reference: bone_index_reference,
                    bone_index_compressed: bone_index_compressed,
                    bone_index: bone_index,
                    compressed_size: compressed_size,
                }
            )
        )
    }

    pub fn decode_helper_point(input: &[u8]) -> IResult<&[u8], BbmHelperPoint, Error> {
        let (input, matrix) = count(le_f32, 4usize)(input)?;
        let (input, hierarchy) = le_i32(input)?;

        Ok(
            (
                input,
                BbmHelperPoint {
                    matrix: matrix,
                    hierarchy: hierarchy,
                }
            )
        )
    }

    pub fn decode_helper_dummy(input: &[u8]) -> IResult<&[u8], BbmHelperDummy, Error> {
        let (input, matrix) = count(le_f32, 4usize)(input)?;
        let (input, hierarchy) = le_i32(input)?;

        Ok(
            (
                input,
                BbmHelperDummy {
                    matrix: matrix,
                    hierarchy: hierarchy,
                }
            )
        )
    }
}