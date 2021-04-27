use quick_xml::events::Event as XmlEvent;

use crate::BadPos;

pub struct DefXml {
    control_structures: Vec<DefXmlControl>,
    definition_types: Vec<DefXmlDefinitionType>,
}

pub struct DefXmlControl {
    name: String,
    id: u32,
    items: Vec<DefXmlItem>,
}

pub enum DefXmlItem {
    Member {
        name: String,
        typ: DefXmlType,
        items: Vec<DefXmlItem>,
    },
    Array {
        name: String,
        elementcount: String,
        items: Vec<DefXmlItem>,
    },
    Link {
        to: String,
        restrictions: String,
    }
}

pub enum DefXmlType {

}

pub struct DefXmlDefinitionType {

}

impl DefXml {
    pub fn new() -> Result<DefXml, BadPos> {
        Self::decode(include_bytes!("./def.xml"))
    }

    pub fn decode(mut data: &[u8]) -> Result<DefXml, BadPos> {
        let data = std::str::from_utf8(data).or(Err(BadPos))?;

        let mut out = DefXml {
            control_structures: Vec::with_capacity(1024),
            definition_types: Vec::with_capacity(128),
        };

        let mut reader = quick_xml::Reader::from_str(data);
        let mut event_buf = Vec::new();

        loop {
            match reader.read_event(&mut event_buf) {
                Ok(XmlEvent::Start(e)) => {
                    println!("{:?}", e.name());
                },
                Ok(XmlEvent::Text(e)) => {
                    eprintln!("Found text at {}: {:?}", reader.buffer_position(), e);
                    return Err(BadPos)
                },
                Err(e) => {
                    eprintln!("Error at {}: {:?}", reader.buffer_position(), e);
                    return Err(BadPos)
                },
                Ok(XmlEvent::Eof) => {
                    break
                },
                _ => (),
            }
        }

        Ok(out)
    }
}