use std::io::Read;

use crate::Error;

use crate::{
    IResult,
    alt,
    count,
    decode_rle_string,
    le_f32,
    le_u16,
    le_u32,
    le_u64,
    le_u8,
    tag,
    take,
    tuple,
};

/// Level cells and nodes.
///
/// # Format Description
///
/// WIP
#[derive(Debug,PartialEq)]
pub struct Lev {
    pub header: LevHeader,
    pub heightmap_cells: Vec<LevHeightCell>,
    pub soundmap_cells: Vec<LevSoundCell>,
    // pub navigation_header: LevNavigationHeader,
    // pub navigation_section: LevNavigationSection
}

#[derive(Debug,PartialEq)]
pub struct LevHeader {
    pub version: u16,
    pub obsolete_offset: u32,
    pub navigation_offset: u32,
    pub unique_id_count: u64,
    pub width: u32,
    pub height: u32,
    pub map_version: u32,
    // pub heightmap_palette: &'a [u8],
    pub ambient_sound_version: u32,
    // pub sound_palette: &'a [u8],
    pub checksum: u32,
    pub sound_themes: Vec<String>,
}


#[derive(Debug,PartialEq)]
pub struct LevHeightCell {
    pub size: u32,
    pub version: u8,
    pub height: f32,
    pub ground_theme: (u8, u8, u8),
    pub ground_theme_strength: (u8, u8),
    pub walkable: bool,
    pub passover: bool,
    pub sound_theme: u8,
    pub shore: bool,
}

#[derive(Debug,PartialEq)]
pub struct LevSoundCell {
    pub size: u32,
    pub version: u8,
    pub sound_theme: (u8, u8, u8),
    pub sound_theme_strength: (u8, u8),
    pub sound_index: u8,
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationHeader {
    pub sections_start: u32,
    pub sections_count: u32,
    pub sections: Vec<(String, u32)>,
}

//
// From fabletlcmod.com:
//
// A Subset has 7 Layers (0-6), each defining blocks of walkable area.
// Layer 0 = 32 X 32
// Layer 1 = 16 X 16
// Layer 2 = 8 X 8
// Layer 3 = 4 X 4
// Layer 4 = 2 X 2
// Layer 5 = 1 X 1
// Layer 6 = 0.5 X 0.5
//

#[derive(Debug,PartialEq)]
pub struct LevNavigationSection {
    pub size: u32,
    pub version: u32,
    pub level_width: u32,
    pub level_height: u32,
    pub interactive_nodes: Vec<LevInteractiveNode>,
    pub subsets_count: u32,
    pub level_nodes: Vec<LevNavigationNode>,
}

#[derive(Debug,PartialEq)]
pub struct LevInteractiveNode {
    pub x: u32,
    pub y: u32,
    pub subset: u32,
}

#[derive(Debug,PartialEq)]
pub enum LevNavigationNode {
    Regular(LevNavigationRegularNode),
    Navigation(LevNavigationNavigationNode),
    Exit(LevNavigationExitNode),
    Blank(LevNavigationBlankNode),
    Unknown1(LevNavigationUnknownNode1),
    Unknown2(LevNavigationUnknownNode2),
    Unknown3(LevNavigationUnknownNode3),
    Unknown4(LevNavigationUnknownNode4),
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationRegularNode {
    pub root: u8,
    pub end: u8,
    pub layer: u8,
    pub subset: u8,
    pub x: f32,
    pub y: f32,
    pub node_id: u32,
    pub child_nodes: (u32, u32, u32, u32) // (top_right, top_left, bottom_right, bottom_left)
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationNavigationNode {
    pub root: u8,
    pub end: u8,
    pub layer: u8,
    pub subset: u8,
    pub x: f32,
    pub y: f32,
    pub node_id: u32,
    pub node_level: u32,
    pub nodes: Vec<u32>,
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationExitNode {
    pub root: u8,
    pub end: u8,
    pub layer: u8,
    pub subset: u8,
    pub x: f32,
    pub y: f32,
    pub node_id: u32,
    pub node_level: u32,
    pub nodes: Vec<u32>,
    pub uids: Vec<u64>,
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknownNode1 {
    pub node_op: Vec<u8>,
    pub end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknownNode2 {
    pub end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknownNode3 {
    pub end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknownNode4 {
    pub end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationBlankNode {
    pub root: u8
}

impl Lev {
    pub fn decode<Source: Read>(source: &mut Source) -> Result<Lev, Error> {
        let mut data = Vec::new();
        source.read_to_end(&mut data)?;
        // let (_, lev) = all_consuming(Lev::decode_lev)(&data)?;
        let (_, lev) = Lev::decode_lev(&data)?;
        Ok(lev)
    }

    pub fn decode_lev(input: &[u8]) -> IResult<&[u8], Lev, Error> {
        let (input, header) = Self::decode_header(input)?;

        let heightmap_cell_count = ((header.width + 1) * (header.height + 1)) as usize;
        let (input, heightmap_cells) = count(Self::decode_heightmap_cell, heightmap_cell_count)(input)?;

        // fabletlcmod.com seems to have the wrong amount, using a temporary one.
        // let soundmap_cell_count = ((header.height - 1) * (header.width - 1)) as usize;
        let soundmap_cell_count = 1024usize;
        let (_input, soundmap_cells) = count(Self::decode_soundmap_cell, soundmap_cell_count)(input)?;

        // let navigation_header_data = &input[header.navigation_offset as usize..];
        // let (_input, navigation_header) = Self::decode_navigation_header(navigation_header_data)?;

        // let navigation_section_data = &input[navigation_header.sections_start as usize..];
        // let (input, navigation_section) = Self::decode_navigation_section(navigation_section_data)?;

        Ok(
            (
                input,
                Lev {
                    header: header,
                    heightmap_cells: heightmap_cells,
                    soundmap_cells: soundmap_cells,
                    // navigation_header: navigation_header,
                    // navigation_section: navigation_section,
                }
            )
        )
    }

    pub fn decode_header(input: &[u8]) -> IResult<&[u8], LevHeader, Error> {
        let (input, _header_size) = le_u32(input)?;
        let (input, version) = le_u16(input)?;
        let (input, _unknown_1) = take(3usize)(input)?; // fabletlcmod.com: 3 bytes of padding. see checksum.
        let (input, _unknown_2) = le_u32(input)?;
        let (input, obsolete_offset) = le_u32(input)?;
        let (input, _unknown_3) = le_u32(input)?;
        let (input, navigation_offset) = le_u32(input)?;
        let (input, _map_header_size) = le_u8(input)?;
        let (input, map_version) = le_u32(input)?; // fabletlcmod.com:  An 8 bit integer (with 3 bytes of padding)
        let (input, unique_id_count) = le_u64(input)?;
        let (input, width) = le_u32(input)?;
        let (input, height) = le_u32(input)?;
        let (input, _always_true) = le_u8(input)?;

        let (input, _heightmap_palette) = take(33792usize)(input)?; // TODO: figure this out
        let (input, ambient_sound_version) = le_u32(input)?;
        let (input, sound_themes_count) = le_u32(input)?;
        let (input, _sound_palette) = take(33792usize)(input)?; // TODO: figure this out
        let (input, checksum) = le_u32(input)?; // fabletlcmod.com: only if the map header pad byte 2 is 9.

        let (input, sound_themes) = count(decode_rle_string, (sound_themes_count - 1) as usize)(input)?;

        Ok(
            (
                input,
                LevHeader {
                    version: version,
                    obsolete_offset: obsolete_offset,
                    navigation_offset: navigation_offset,
                    unique_id_count: unique_id_count,
                    width: width,
                    height: height,
                    map_version: map_version,
                    // heightmap_palette: heightmap_palette,
                    ambient_sound_version: ambient_sound_version,
                    // sound_palette: sound_palette,
                    checksum: checksum,
                    sound_themes: sound_themes,
                }
            )
        )
    }

    pub fn decode_heightmap_cell(input: &[u8]) -> IResult<&[u8], LevHeightCell, Error> {
        let (input, size) = le_u32(input)?;
        let (input, version) = le_u8(input)?;
        let (input, height) = le_f32(input)?;
        let (input, _zero) = le_u8(input)?;
        let (input, ground_theme) = tuple((le_u8, le_u8, le_u8))(input)?;
        let (input, ground_theme_strength) = tuple((le_u8, le_u8))(input)?;
        let (input, walkable) = le_u8(input)?;
        let (input, passover) = le_u8(input)?;
        let (input, sound_theme) = le_u8(input)?;
        let (input, _zero) = le_u8(input)?;
        let (input, shore) = le_u8(input)?;
        let (input, _unknown) = le_u8(input)?;

        Ok(
            (
                input,
                LevHeightCell {
                    size: size,
                    version: version,
                    height: height,
                    ground_theme: ground_theme,
                    ground_theme_strength: ground_theme_strength,
                    walkable: walkable != 0,
                    passover: passover != 0,
                    sound_theme: sound_theme,
                    shore: shore != 0,
                }
            )
        )
    }

    pub fn decode_soundmap_cell(input: &[u8]) -> IResult<&[u8], LevSoundCell, Error> {
        let (input, size) = le_u32(input)?;
        let (input, version) = le_u8(input)?;
        let (input, sound_theme) = tuple((le_u8, le_u8, le_u8))(input)?;
        let (input, sound_theme_strength) = tuple((le_u8, le_u8))(input)?;
        let (input, sound_index) = le_u8(input)?;

        Ok(
            (
                input,
                LevSoundCell {
                    size: size,
                    version: version,
                    sound_theme: sound_theme,
                    sound_theme_strength: sound_theme_strength,
                    sound_index: sound_index,
                }
            )
        )
    }

    pub fn decode_navigation_header(input: &[u8]) -> IResult<&[u8], LevNavigationHeader, Error> {
        let (input, sections_start) = le_u32(input)?;
        let (input, sections_count) = le_u32(input)?;

        let (input, sections) = count(Self::decode_navigation_header_section, sections_count as usize)(input)?;

        Ok(
            (
                input,
                LevNavigationHeader {
                    sections_start: sections_start,
                    sections_count: sections_count,
                    sections: sections,
                }
            )
        )
    }

    pub fn decode_navigation_header_section(input: &[u8]) -> IResult<&[u8], (String, u32), Error> {
        let (input, name) = decode_rle_string(input)?;
        let (input, start) = le_u32(input)?;

        Ok( (input, (name, start)) )
    }


    pub fn decode_navigation_section(input: &[u8]) -> IResult<&[u8], LevNavigationSection, Error> {
        let (input, size) = le_u32(input)?;
        let (input, version) = le_u32(input)?;
        let (input, level_width) = le_u32(input)?;
        let (input, level_height) = le_u32(input)?;
        let (input, _unknown_1) = le_u32(input)?; // fabletlcmod.com: Number of levels, see navigation nodes

        let (input, interactive_nodes_count) = le_u32(input)?;
        let (input, interactive_nodes) = count(Self::decode_navigation_interactive_node, interactive_nodes_count as usize)(input)?;

        let (input, subsets_count) = le_u32(input)?;

        // println!("size {:#?}", size);
        // println!("version {:#?}", version);
        // println!("level_width {:#?}", level_width);
        // println!("level_height {:#?}", level_height);
        // println!("interactive_nodes_count {:#?}", interactive_nodes_count);
        // println!("interactive_nodes {:#?}", interactive_nodes);
        // println!("subsets_count {:#?}", subsets_count);

        let (input, level_nodes_count) = le_u32(input)?;
        println!("level_nodes_count {:#?}", level_nodes_count);
        let (input, level_nodes) = count(Self::decode_navigation_level_node, level_nodes_count as usize)(input)?;
        // let (input, level_nodes) = count(decode_navigation_level_node, 1usize)(input)?;
        // println!("level_nodes {:?}", level_nodes);

        Ok(
            (
                input,
                LevNavigationSection {
                    size: size,
                    version: version,
                    level_width: level_width,
                    level_height: level_height,
                    interactive_nodes: interactive_nodes,
                    subsets_count: subsets_count,
                    level_nodes: level_nodes,
                }
            )
        )
    }

    pub fn decode_navigation_interactive_node(input: &[u8]) -> IResult<&[u8], LevInteractiveNode, Error> {
        let (input, x) = le_u32(input)?;
        let (input, y) = le_u32(input)?;
        let (input, subset) = le_u32(input)?;

        Ok(
            (
                input,
                LevInteractiveNode {
                    x: x,
                    y: y,
                    subset: subset,
                }
            )
        )
    }

    pub fn decode_navigation_level_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        // println!("next node {:?}", &input[..18]);

        let (input, level_node) = alt((
            Self::decode_navigation_regular_node,
            Self::decode_navigation_navigation_node,
            Self::decode_navigation_exit_node,
            Self::decode_navigation_blank_node,
            Self::decode_navigation_unknown1_node,
            Self::decode_navigation_unknown2_node,
            Self::decode_navigation_unknown3_node,
            Self::decode_navigation_unknown_node,
        ))(input)?;

        // println!("level_node {:?}", level_node);

        Ok((input, level_node))
    }

    pub fn decode_navigation_regular_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[0, 0, 0, 0, 0, 1, 0, 0])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (input, end) = le_u8(input)?;
        let (input, layer) = le_u8(input)?;
        let (input, subset) = le_u8(input)?;
        let (input, x) = le_f32(input)?;
        let (input, y) = le_f32(input)?;
        let (input, node_id) = le_u32(input)?;

        println!("node_id {:?}", node_id);

        let (input, child_nodes) = tuple((le_u32, le_u32, le_u32, le_u32))(input)?;

        Ok(
            (
                input,
                LevNavigationNode::Regular(
                    LevNavigationRegularNode {
                        root: root,
                        end: end,
                        layer: layer,
                        subset: subset,
                        x: x,
                        y: y,
                        node_id: node_id,
                        child_nodes: child_nodes,
                    }
                )
            )
        )
    }

    pub fn decode_navigation_navigation_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[0, 0, 0, 1, 0, 1, 0, 1])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (input, end) = le_u8(input)?;
        let (input, layer) = le_u8(input)?;
        let (input, subset) = le_u8(input)?;
        let (input, x) = le_f32(input)?;
        let (input, y) = le_f32(input)?;
        let (input, node_id) = le_u32(input)?;

        println!("node_id {:?}", node_id);

        let (input, node_level) = le_u32(input)?; // fabletlcmod.com: Represents some sort of z level attribute
        let (input, _unknown_3) = le_u8(input)?;  // fabletlcmod.com: So far, Subset 0 = 0 or 128, SubSet 1+ = 64

        let (input, nodes_count) = le_u32(input)?;
        let (input, nodes) = count(le_u32, nodes_count as usize)(input)?;

        Ok(
            (
                input,
                LevNavigationNode::Navigation(
                    LevNavigationNavigationNode {
                        root: root,
                        end: end,
                        layer: layer,
                        subset: subset,
                        x: x,
                        y: y,
                        node_id: node_id,
                        node_level: node_level,
                        nodes: nodes,
                    }
                )
            )
        )
    }

    pub fn decode_navigation_exit_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[1, 0, 0, 1, 1, 0, 1, 1])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (input, end) = le_u8(input)?;
        let (input, layer) = le_u8(input)?;
        let (input, subset) = le_u8(input)?;
        let (input, x) = le_f32(input)?;
        let (input, y) = le_f32(input)?;
        let (input, node_id) = le_u32(input)?;

        println!("node_id {:?}", node_id);

        let (input, node_level) = le_u32(input)?; // fabletlcmod.com: Represents some sort of z level attribute
        let (input, _unknown_3) = le_u8(input)?;  // fabletlcmod.com: So far, Subset 0 = 0 or 128, SubSet 1+ = 64

        let (input, nodes_count) = le_u32(input)?;
        let (input, nodes) = count(le_u32, nodes_count as usize)(input)?;

        // fabletlcmod.com: Stripped UID to create the real uid add 18446741874686296064
        let (input, uids_count) = le_u32(input)?;
        let (input, uids) = count(le_u64, uids_count as usize)(input)?;

        Ok(
            (
                input,
                LevNavigationNode::Exit(
                    LevNavigationExitNode {
                        root: root,
                        end: end,
                        layer: layer,
                        subset: subset,
                        x: x,
                        y: y,
                        node_id: node_id,
                        node_level: node_level,
                        nodes: nodes,
                        uids: uids,
                    }
                )
            )
        )
    }

    pub fn decode_navigation_unknown1_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[11, 0, 0, 0, 0, 0, 0, 0, 0, 0])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, _root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (_input, end) = le_u8(input)?;

        let unknown = LevNavigationUnknownNode2 {
            end: end,
        };

        let input = &input[end as usize..];

        Ok((input, LevNavigationNode::Unknown2(unknown)))
    }

    pub fn decode_navigation_unknown2_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[0, 1, 0, 0, 0, 0, 0, 0])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, _root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (_input, end) = le_u8(input)?;

        let unknown = LevNavigationUnknownNode3 {
            end: end,
        };

        let input = &input[end as usize..];

        Ok((input, LevNavigationNode::Unknown3(unknown)))
    }

    pub fn decode_navigation_unknown3_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[0, 0, 0, 0, 0, 0, 0, 0])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, _root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (_input, end) = le_u8(input)?;

        let unknown = LevNavigationUnknownNode4 {
            end: end,
        };

        let input = &input[end as usize..];

        Ok((input, LevNavigationNode::Unknown4(unknown)))
    }

    pub fn decode_navigation_unknown_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, node_op) = take(8usize)(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, _root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;
        let (_input, end) = le_u8(input)?;

        let unknown = LevNavigationUnknownNode1 {
            node_op: node_op.to_vec(),
            end: end,
        };

        let input = &input[end as usize..];

        Ok((input, LevNavigationNode::Unknown1(unknown)))
    }

    pub fn decode_navigation_blank_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode, Error> {
        let (input, _node_op) = tag(&[0, 1, 1])(input)?;
        let (input, _unknown_1) = le_u8(input)?;
        let (input, root) = le_u8(input)?;
        let (input, _unknown_2) = le_u8(input)?;

        println!("blank {:?}", root);

        Ok(
            (
                input,
                LevNavigationNode::Blank(
                    LevNavigationBlankNode {
                        root: root,
                    }
                )
            )
        )
    }
}