use std::{error::Error, fmt::Write, path::Path};

use compio::fs;

#[compio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    visit_dirs(".", &mut output).await?;

    _ = fs::write("./tree.csv", output).await;
    Ok(())
}

async fn visit_dirs(path: impl AsRef<Path>, mut out: impl Write) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();

    match path_str.as_ref() {
        "tree.csv" | "tree.exe" => {
            return Ok(());
        }
        _ => (),
    }

    let Ok(meta) = fs::metadata(path).await else {
        return Ok(());
    };

    if meta.is_dir() {
        Box::pin(visit_dirs(path, out)).await?;
    } else {
        let file_size = meta.len();
        out.write_fmt(format_args!("{path_str},{file_size}\n"))?;
    }

    Ok(())
}
