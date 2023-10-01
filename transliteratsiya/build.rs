use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

type Mapping = (String, String);

#[derive(Debug)]
struct Standard {
    mapping: PathBuf,
    pre_processor_mapping: Option<PathBuf>,
    reverse_specific_mapping: Option<PathBuf>,
    reverse_specific_pre_processor_mapping: Option<PathBuf>,
}

fn list_languages() -> Vec<String> {
    let mut languages = vec![];
    let paths = std::fs::read_dir("data").expect("Should locate data directory");

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            languages.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    languages
}

fn read_standard(dir_name: &Path, language: &str) -> Standard {
    let mapping_file = dir_name.join(language).join("mapping.csv");
    let pre_processor_mapping_file = dir_name.join(language).join("pre_processor_mapping.csv");
    let reverse_specific_mapping_file =
        dir_name.join(language).join("reverse_specific_mapping.csv");
    let reverse_specific_pre_processor_mapping_file = dir_name
        .join(language)
        .join("reverse_specific_pre_processor_mapping.csv");

    let mapping = match mapping_file.try_exists() {
        Ok(true) => mapping_file,
        Ok(false) | Err(_) => panic!("mapping.csv for {} not found", language),
    };

    let pre_processor_mapping = match pre_processor_mapping_file.try_exists() {
        Ok(true) => Some(pre_processor_mapping_file),
        Ok(false) | Err(_) => None,
    };

    let reverse_specific_mapping = match reverse_specific_mapping_file.try_exists() {
        Ok(true) => Some(reverse_specific_mapping_file),
        Ok(false) | Err(_) => None,
    };

    let reverse_specific_pre_processor_mapping =
        match reverse_specific_pre_processor_mapping_file.try_exists() {
            Ok(true) => Some(reverse_specific_pre_processor_mapping_file),
            Ok(false) | Err(_) => None,
        };

    Standard {
        mapping,
        pre_processor_mapping,
        reverse_specific_mapping,
        reverse_specific_pre_processor_mapping,
    }
}

fn write_standard(dir_name: &Path, standard: Standard) {
    let mapping_in = dir_name.join("mapping.in");
    let pre_processor_mapping_in = dir_name.join("pre_processor_mapping.in");
    let reverse_specific_mapping_in = dir_name.join("reverse_specific_mapping.in");
    let reverse_specific_pre_processor_mapping_in =
        dir_name.join("reverse_specific_pre_processor_mapping.in");

    let mut module = File::create(mapping_in).unwrap();
    write_standard_in_file(&standard.mapping, &mut module).unwrap();

    if let Some(pre_processor_mapping) = &standard.pre_processor_mapping {
        let mut module = File::create(pre_processor_mapping_in).unwrap();
        write_standard_in_file(pre_processor_mapping, &mut module).unwrap();
    }

    if let Some(reverse_specific_mapping) = &standard.reverse_specific_mapping {
        let mut module = File::create(reverse_specific_mapping_in).unwrap();
        write_standard_in_file(reverse_specific_mapping, &mut module).unwrap();
    }

    if let Some(reverse_specific_pre_processor_mapping) =
        &standard.reverse_specific_pre_processor_mapping
    {
        let mut module = File::create(reverse_specific_pre_processor_mapping_in).unwrap();
        write_standard_in_file(reverse_specific_pre_processor_mapping, &mut module).unwrap();
    }
}

fn read_data(file: &Path) -> Vec<Mapping> {
    let mut data = vec![];

    let file = File::open(file).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(file);

    for record in rdr.records() {
        let record = record.unwrap();

        data.push((
            record.get(0).unwrap().to_string(),
            record.get(1).unwrap().to_string(),
        ));
    }

    data
}

fn write_standard_in_file(in_file: &Path, out_file: &mut File) -> Result<(), std::io::Error> {
    let mappings = read_data(in_file);

    writeln!(out_file, "[")?;

    for (a, b) in mappings {
        writeln!(out_file, r#"    ("{}", "{}"),"#, a.as_str(), b.as_str())?;
    }

    writeln!(out_file, "]")?;

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let data_dir = Path::new("data");

    let languages = list_languages();

    for language in languages {
        let standard = read_standard(data_dir, &language);

        let dir_name = format!("src/standards/{}", language);
        let dir_name = Path::new(&dir_name);
        create_dir_all(dir_name).unwrap();

        write_standard(dir_name, standard);
    }
}
