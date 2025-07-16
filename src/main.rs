use std::{error::Error, fmt::Write, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    visit_dirs(".", &mut output)?;

    _ = fs::write("./tree.csv", output.replace("\\", "/"));
    Ok(())
}

fn visit_dirs(dirpath: impl AsRef<Path>, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let path = dirpath.as_ref();

    let results = fs::read_dir(path)?;

    for entry in results.flatten() {
        let meta = entry.metadata()?;
        let name = entry.file_name();
        let path = path.join(&name);

        if meta.is_dir() {
            visit_dirs(path, out)?;
            continue;
        }

        let length = meta.len();
        let name = name.to_string_lossy();
        match &*name {
            "tree.exe" | "tree.csv" => continue,
            _ => (),
        }

        out.write_fmt(format_args!("{},{length}\n", path.to_string_lossy()))?;
    }

    Ok(())
}
