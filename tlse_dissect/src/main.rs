use std::fs::read;

use pelite::pe32::{Pe,PeFile};

use capstone::prelude::*;
use capstone::Capstone;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let fable_exe_path = args.get(0).unwrap();
    let fable_exe = read(fable_exe_path).unwrap();

    let pe_file = PeFile::from_bytes(&fable_exe).unwrap();

    let optional_header = pe_file.optional_header();
    let image_base = optional_header.ImageBase;

    let text_section_header = pe_file
        .section_headers()
        .iter()
        .find(|x| x.Name.to_str().unwrap() == ".text")
        .expect("No .text section found.");

    let text_section = pe_file.get_section_bytes(text_section_header).unwrap();

    let cs = Capstone::new()
        .x86()
        .mode(capstone::arch::x86::ArchMode::Mode32)
        .syntax(capstone::arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Failed to initialize capstone.");

    let insns = cs.disasm_all(text_section, image_base as u64).unwrap();

    for i in insns.iter() {
        println!("{}", i);

        // let insn_detail = cs.insn_detail(&i).unwrap();

        // for group in insn_detail.groups() {
        //     let group_name =
        //     println!("{}", i);
        // }
    }
}