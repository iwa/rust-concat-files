use std::fs;

fn main() {
    println!("Generating one compacted json file...");

    let directory = "./data";
    let output_file = "output.json";
    let mut content = String::new();

    println!("Reading files from directory: {}", directory);
    let files_list = list_files_in_directory(directory);

    for file in files_list {
        println!("  Reading file: {}", file);
        let file_content = read_file(&file);
        content.push_str(&file_content);
        content.push('\n');
        content.push('\n');
    }

    println!("Writing to output file: {}", output_file);
    write_file(output_file, &content);

    println!("Done!");
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Unable to read file")
}

fn write_file(file_path: &str, content: &str) {
    fs::write(file_path, content).expect("Unable to write file");
}

fn list_files_in_directory(directory: &str) -> Vec<String> {
    let paths = fs::read_dir(directory).expect("Unable to read directory");
    paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.ends_with(".json"))
                .unwrap_or(false)
        })
        .map(|entry| entry.path().display().to_string())
        .collect()
}
