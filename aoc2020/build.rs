use std::fs;
use std::fs::DirEntry;

fn main() {
    let output_path = std::env::var("OUT_DIR").expect("No output dir?");
    // Find the library with the biggest alphabetical name

    let src_dir = fs::read_dir("src")
        .expect("No source directory?")
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();

    if let Some(build_content) = [get_furthest(&src_dir)]
        .into_iter()
        .collect::<Option<Vec<_>>>()
    {
        fs::write(
            std::path::Path::new(&output_path).join("data_includes.rs"),
            build_content.join("\n"),
        )
        .expect("Could not write build data to file?");
    } else {
        eprintln!("No data_includes generated, failed...");
        fs::remove_file(output_path).expect("Could not remove outdated build data file?");
        panic!("Failed ot create data_includes")
    }
}

fn get_furthest(lib_dir: &[DirEntry]) -> Option<String> {
    Some(format!(
        "pub const FURTHEST_DAY: usize = {};",
        lib_dir
            .iter()
            .filter_map(|e| e.file_name().into_string().ok()) // dir name
            .map(|name| name.trim_start_matches("day_").to_string())
            .filter_map(|num| num.parse::<usize>().ok())
            .max()?
    ))
}