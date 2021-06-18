use lalrpop_util::lalrpop_mod;

lalrpop_mod!(ini_parser, "/ini_parser.rs");

pub use ini_parser::IniParser;

pub struct Ini {}

impl Ini {
    // pub fn decode<Source: Read + Seek>(source: &mut Source) -> Result<Self, Error> {
    //     todo!()
    // }
}
