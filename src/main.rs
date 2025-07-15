use std::{error::Error, fmt::Write, fs::read_dir, path::Path};

use compio::{fs, runtime::spawn_blocking};
use futures_util::{StreamExt, stream};

#[compio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let cpu_count = num_cpus::get();
    visit_dirs(".", &mut output, cpu_count).await?;

    _ = fs::write("./tree.csv", output.replace("\\", "/")).await;
    Ok(())
}

async fn visit_dirs(
    path: impl AsRef<Path>,
    out: &mut impl Write,
    cpu_count: usize,
) -> Result<(), Box<dyn Error>> {
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
        let path = path.to_path_buf();
        let results = spawn_blocking(move || read_dir(path))
            .await
            .unwrap()?
            .flatten()
            .map(|e| e.path());
        let new = stream::iter(results)
            .map(async |path| {
                let mut out = String::new();
                _ = visit_dirs(path, &mut out, cpu_count).await;
                out
            })
            .buffer_unordered(cpu_count)
            .collect::<String>()
            .await;
        out.write_str(&new)?;
    } else {
        let file_size = meta.len();
        out.write_fmt(format_args!("{path_str},{file_size}\n"))?;
    }

    Ok(())
}
