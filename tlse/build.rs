fn main() {
    // let profile = std::env::var("PROFILE").unwrap();

    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let build_dir_path = std::path::Path::new(&out_dir).parent().unwrap();
    // let build_dir = build_dir_path.to_str().unwrap();

    // let cwd_path = std::env::current_dir().unwrap();
    // let cwd = cwd_path.to_str().unwrap();
    // let out = format!("target\\{}", profile);

    // let mut cmd = cc::Build::new()
    //     .cpp(true)
    //     .include("src")
    //     .flag("-LD")
    //     .flag("-std:c++17")
    //     .flag("-MACHINE:X86")
    //     .flag(&format!("-Fetarget\\{}\\{}", profile, "tlse_hack.dll"))
    //     .flag(&format!("-Fotarget\\{}\\{}", profile, "tlse_hack.obj"))
    //     .get_compiler()
    //     .to_command();

    // cmd.arg("src/tlse_hack.cc");

    // println!("cargo:warning=Building C++ with");
    // println!("cargo:warning={:?}", cmd);

    // let mut child = cmd.spawn().unwrap();

    // child.wait().unwrap();

    // println!("cargo:warning=To see the compiler output check {}", build_dir);
}