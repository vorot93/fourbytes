use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

fn main() {
    let mut data_path = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("4bytes");
    data_path.push("signatures");

    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_path.push("signatures.rs");

    let out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_path)
        .unwrap();

    let mut out_file = BufWriter::new(out_file);

    write!(
        out_file,
        "pub static SIGNATURES: Signatures = Signatures(phf::phf_map! {{"
    )
    .unwrap();

    for entry in fs::read_dir(data_path).unwrap() {
        let entry = entry.unwrap();

        write!(
            out_file,
            "{}_u32 => \"",
            u32::from_str_radix(&entry.file_name().into_string().unwrap(), 16).unwrap()
        )
        .unwrap();
        std::io::copy(
            &mut std::fs::File::open(entry.path()).unwrap(),
            &mut out_file,
        )
        .unwrap();
        write!(out_file, "\",").unwrap();
    }

    write!(out_file, "}});").unwrap();

    out_file.flush().unwrap();
}
