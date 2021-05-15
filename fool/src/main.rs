use std::collections::HashSet;
use std::path::{PathBuf,Component};
use std::fs::{read,write,create_dir_all,File};
use std::io::{Read,BufReader};
use std::ffi::OsStr;

use pico_args::Arguments;

use fable_data::{Wad,Big,BigInfo,Texture,Lev,Stb,StbLev,NamesBin,Bin,Mesh};



fn main() {
    let mut args = Arguments::from_env();

    let source: String = args.free_from_str().unwrap();

    let source_path = PathBuf::from(source);

    let file_type = source_path.extension()
        .map(|x| x.to_str()).flatten().map(|x| x.to_owned())
        .or_else(|| args.opt_free_from_str().unwrap_or(None));

    match file_type.as_deref() {
        Some("wad") => wad(args, source_path),
        Some("big") | Some("fmp") => big(args, source_path),
        Some("lev") => wad_lev(args, source_path),
        Some("stb") => stb(args, source_path),
        Some("stb_lev") => stb_lev(args, source_path),
        Some("bbm") => bbm(args, source_path),
        Some("bin") => names_bin(args, source_path),
        Some("tex") => tex(args, source_path),
        _ => panic!("File has unhandled extension and no flag was given to specify the format."),
    }
}

fn wad(mut args: Arguments, path: PathBuf) {
    let mut wad_file = BufReader::new(File::open(&path).unwrap());

    let wad = Wad::decode(&mut wad_file).unwrap();

    if args.contains(["-d","--debug"]) {
        println!("{:#?}", wad);
        return
    }

    let wad_name = path.file_stem().unwrap();

    for entry in wad.entries {
        let mut file_data = vec![0; entry.data_size as usize];

        entry.read_from(&mut wad_file, &mut file_data).unwrap();

        let file_path = PathBuf::from(entry.path);

        let file_path_components: Vec<Component<'_>> = file_path
            .components()
            .skip_while(|x| *x != Component::Normal(OsStr::new(wad_name)))
            .skip(1)
            .collect();

        let file_path = if file_path_components.len() > 0 {
            let file_path_relevant: PathBuf = file_path_components.iter().collect();
            path.with_extension("").join(file_path_relevant)
        } else {
            path.parent().unwrap().join(file_path.file_name().unwrap())
        };

        match create_dir_all(file_path.parent().unwrap()) {
            Ok(_) => {}
            Err(x) if x.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(x) => panic!("{}", x),
        };

        write(file_path, file_data).unwrap();
    }
}

fn stb(mut args: Arguments, path: PathBuf) {
    let mut file = BufReader::new(File::open(&path).unwrap());

    let stb = Stb::decode(&mut file).unwrap();

    let debug_mode = args.contains(["-d","--debug"]);

    println!("Hello world");

    match args.opt_free_from_str::<String>().unwrap() {
        Some(entry_path) => {
            println!("{:?}", entry_path);
            for entry in stb.entries {
                if entry.name_1 == entry_path {
                    // Stb::decode_static_map_common();
                    println!("{:#?}", entry);
                }
            }
        }
        None => {
            println!("Hello world 2");

            if debug_mode {
                println!("{:#?}", stb);
                return
            }

            let stb_name = path.file_stem().unwrap();

            for entry in stb.entries {
                let mut file_data = vec![0; entry.len as usize];

                entry.read_from(&mut file, &mut file_data).unwrap();

                let file_path = PathBuf::from(stb_name).join(&entry.name_1);

                let file_path_components: Vec<Component<'_>> = file_path
                    .components()
                    .skip(1)
                    .collect();

                let mut file_path = if file_path_components.len() > 0 {
                    let file_path_relevant: PathBuf = file_path_components.iter().collect();
                    path.with_extension("").join(file_path_relevant)
                } else {
                    path.parent().unwrap().join(file_path.file_name().unwrap())
                };

                // TODO: Not really necessary, but helps distinguish from wad levs.
                if file_path.extension().map(|x| x.to_str()).flatten() == Some("lev") {
                    file_path.set_extension("stb_lev");
                }

                match create_dir_all(file_path.parent().unwrap()) {
                    Ok(_) => {}
                    Err(x) if x.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(x) => panic!("{}", x),
                };

                write(file_path, file_data).unwrap();
            }
        }
    }
}

fn big(mut args: Arguments, path: PathBuf) {
    let mut big_file = BufReader::new(File::open(&path).unwrap());

    let big = Big::decode_reader_with_path(&mut big_file, &path).unwrap();
    let big_index = big.index_by_name();

    if let Ok(Some(bank_name)) = args.opt_free_from_str::<String>() {
        if let Some(bank) = big_index.get(&bank_name) {
            let bank_index = bank.index_by_name();

            match args.opt_free_from_str::<String>() {
                Ok(Some(entry_name)) => {
                    if let Some(entry) = bank_index.get(&entry_name) {
                        let mut data = vec![0; entry.data_size as usize];

                        entry.read_from(&mut big_file, &mut data).unwrap();

                        match &entry.info {
                            BigInfo::Texture(info) => {
                                // println!("{:?} {:#?}", entry.name, info);
                                let tex = Texture::decode(&data, info).unwrap();
                            },
                            BigInfo::Mesh(info) => {
                                let mesh = Mesh::decode(&data, info).unwrap();
                                // println!("{:#?}\n{:#?}", info, mesh);
                            }
                            _ => {}
                        }
                    } else {
                        eprintln!("Entry not found.");
                    }
                },
                Ok(None) => {
                    for entry in bank.entries.iter() {
                        let mut data = vec![0; entry.data_size as usize];

                        entry.read_from(&mut big_file, &mut data).unwrap();

                        match &entry.info {
                            BigInfo::Texture(info) => {
                                // let tex = Texture::decode(&data, info).unwrap();
                            },
                            BigInfo::Mesh(info) => {
                                let mesh = Mesh::decode(&data, info).unwrap();
                                // if
                                //     // mesh.helper_dummies_compressed > 0 &&
                                //     // mesh.helper_dummies_compressed < 80 &&
                                //     // mesh.helper_dummies_count > 1
                                //     // mesh.helper_point_compressed > 0 &&
                                //     // mesh.helper_point_compressed < 2
                                //     // mesh.helper_dummies_compressed == 0 &&
                                //     // mesh.helper_dummies_count == 2
                                // {
                                //     println!("{:?}", mesh.name);
                                // }
                            }
                            _ => {}
                        }
                    }
                    // eprintln!("Entry not found {:?}", entry_name);
                },
                Err(e) => panic!(e),
            }
        } else {
            eprintln!("Bank not found {:?}", bank_name);
        }
    } else {

    }

    // if args.contains(["-d","--debug"]) {
    //     println!("{:#?}", big);
    //     return
    // }

    // if args.contains("--text") {
    //     println!("id, target, script, sound, text");
    //     for bank in big.banks.iter() {
    //         for entry in bank.entries.iter() {
    //             if entry.kind != 26807 {
    //                 // println!("{:?} {:?}", entry.symbol, entry.kind);
    //                 continue
    //             }

    //             // println!("very begining");

    //             let mut entry_data = vec![0; entry.data_size as usize];

    //             entry.read_from(&mut big_file, &mut entry_data).unwrap();

    //             // println!("read entry");

    //             let mut entry_data = &mut entry_data[..];

    //             let u16_data: Vec<u16> = entry_data
    //                 .chunks_exact(2)
    //                 .map(|x| u16::from_le_bytes([ x[0], x[1] ]))
    //                 .take_while(|x| *x != 0)
    //                 .collect();

    //             let _ = entry_data.take((u16_data.len() + 1) * 2);

    //             let text = String::from_utf16(&u16_data).unwrap();
    //             let text = text.replace("\"", "\"\"");

    //             // println!("{}", text);
    //             // print!("\"{}\",", text);

    //             let len = entry_data.grab_u32_le().unwrap();
    //             let script = entry_data.grab_str(len as usize).unwrap().to_owned();
    //             let script = script.replace("\"", "\"\"");

    //             // print!("\"{}\",", script);

    //             let len = entry_data.grab_u32_le().unwrap();
    //             let target = entry_data.grab_str(len as usize).unwrap().to_owned();
    //             let target = target.replace("\"", "\"\"");

    //             // print!("\"{}\",", target);

    //             let len = entry_data.grab_u32_le().unwrap();
    //             let id = entry_data.grab_str(len as usize).unwrap().to_owned();
    //             let id = id.replace("\"", "\"\"");

    //             // print!("\"{}\",", id);

    //             let actions_count = entry_data.grab_u32_le().unwrap();
    //             let mut actions = Vec::new();

    //             for _ in 0..actions_count {
    //                 let id = entry_data.grab_u32_le().unwrap();
    //                 let name = entry_data.grab_str_until_nul().unwrap().to_owned();
    //                 actions.push((id, name));
    //             }

    //             let actions_str = format!("{:?}", actions).replace("\"", "\"\"");

    //             // print!("\"{}\"\n", unknown_1);

    //             // let sound = entry_data.grab_str(len as usize).unwrap().to_owned();
    //             // let sound = sound.replace("\"", "\"\"");

    //             // println!("\"{:?}\", \"{:?}\", \"{:?}\", \"{:?}\", \"{:?}\", \"{:?}\"", text,
    //             // script, target, id, unknown_1, unknown_2);

    //             // println!("\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{:?}\"", text, script,
    //             // target, id, unknown_1, unknown_2);

    //             println!("\"{}\", \"{}\", \"{}\", \"{}\", \"{}\"", id, target, script, actions_str, text);
    //         }
    //     }
    //     return
    // }

    // let mut types_1 = HashSet::new();
    // // let mut types_2 = HashSet::new();

    // for bank in big.banks.iter() {
    //     for entry in bank.entries.iter() {
    //         let mut file_data = vec![0; entry.data_size as usize];

    //         entry.read_from(&mut big_file, &mut file_data).unwrap();

    //         let file_path =
    //             PathBuf::from(&bank.name).join(
    //                 if entry.name.starts_with("[") {
    //                     let path = &entry.name[1 .. entry.name.len() - 1];
    //                     let path = PathBuf::from(path);
    //                     let file_stem = path.file_stem().unwrap();
    //                     // println!("path name {:?}", entry.name);
    //                     file_stem.to_str().unwrap().to_owned()
    //                 } else {
    //                     entry.name.clone()
    //                 }
    //             );

    //             if !types_1.contains(&entry.kind) {
    //                 types_1.insert(entry.kind);
    //                 println!("{:?} {:?}", file_path, entry.kind);
    //             }

    //             // if types_2.contains(&entry.kind_2) {
    //             //     types_2.insert(entry.kind_2);
    //             //     println!("{:?} {:?}", file_path, entry.kind_2);
    //             // }


    //         match create_dir_all(file_path.parent().unwrap()) {
    //             Ok(_) => {}
    //             Err(x) if x.kind() == std::io::ErrorKind::AlreadyExists => {}
    //             Err(x) => panic!("{}", x),
    //         };

    //         write(&file_path, file_data).unwrap();
    //     }
    // }
}

fn wad_lev(_args: Arguments, path: PathBuf) {
    // println!("{:?}", path.file_stem().unwrap());
    // let mut lev_file = BufReader::new(File::open(path).unwrap());
    // let lev = Lev::decode(&mut lev_file).unwrap();
    // println!("{:#?}", lev);
}

fn bbm(_args: Arguments, path: PathBuf) {
    // let mut bbm_file = BufReader::new(File::open(path).unwrap());
    // let mut data = Vec::new();
    // bbm_file.read_to_end(&mut data).unwrap();
    // let _bbm = Mesh::decode(&mut data).unwrap();
}

fn stb_lev(_args: Arguments, path: PathBuf) {
    println!("{:?}", path.file_stem().unwrap());
    let mut stb_lev_file = BufReader::new(File::open(path).unwrap());
    let stb_lev = StbLev::decode(&mut stb_lev_file, 4).unwrap();
    println!("{:#?}", stb_lev);
}

fn names_bin(_args: Arguments, path: PathBuf) {
    // let mut bin_file = BufReader::new(File::open(path).unwrap());
    // let names_bin = NamesBin::decode(&mut bin_file).unwrap();
    // let bin = Bin::decode(&mut bin_file).unwrap();
}

fn tex(_args: Arguments, path: PathBuf) {
    // let mut tex_file = BufReader::new(File::open(path).unwrap());
    // let tex = Tex::decode(&mut tex_file).unwrap();
}