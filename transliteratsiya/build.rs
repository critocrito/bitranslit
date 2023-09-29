use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

type Mapping = (String, String);

#[derive(Debug)]
struct Standard {
    language: String,
    standard: String,
    path: PathBuf,
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

fn list_standards() -> Vec<Standard> {
    let mut standards = vec![];
    let languages = list_languages();

    for language in languages {
        let data_dir = format!("data/{}", language);

        let paths = std::fs::read_dir(data_dir).expect("Should locate data/language directory");

        for entry in paths {
            let entry = entry.unwrap();
            let path = entry.path();
            let extension = path.extension().unwrap();

            if path.is_file() && extension == "csv" {
                let standard = path.file_stem().unwrap().to_string_lossy().to_string();

                standards.push(Standard {
                    language: language.clone(),
                    standard,
                    path,
                });
            }
        }
    }

    standards
}

fn read_standard(file: &Path) -> Vec<Mapping> {
    let mut mappings = vec![];

    let file = File::open(file).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    for record in rdr.records() {
        let record = record.unwrap();

        mappings.push((
            record.get(0).unwrap().to_string(),
            record.get(1).unwrap().to_string(),
        ));
    }

    mappings
}

fn write_standard_in_file(standard: &Standard, file: &mut File) -> Result<(), std::io::Error> {
    let mappings = read_standard(&standard.path);

    writeln!(file, "[")?;

    for (a, b) in mappings {
        writeln!(file, r#"    ("{}", "{}"),"#, a.as_str(), b.as_str())?;
    }

    writeln!(file, "]")?;

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let languages = list_languages();
    let standards = list_standards();

    for standard in &standards {
        println!(
            "cargo:rerun-if-changed={}",
            standard.path.as_os_str().to_str().unwrap()
        );
    }

    create_dir_all("src/standards").unwrap();

    for language in languages {
        let language_standards: Vec<&Standard> = standards
            .iter()
            .filter(|s| s.language == language)
            .collect();

        let module_name = format!("src/standards/{}.in", language);
        let mut module = File::create(&module_name).unwrap();

        for standard in language_standards {
            write_standard_in_file(standard, &mut module).unwrap();
        }
    }
}
