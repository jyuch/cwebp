use clap::Parser;
use image::imageops::FilterType;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Cli {
    /// Input.
    #[clap(short, long)]
    input: PathBuf,
    /// Output.
    #[clap(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Cli::parse();

    for entry in WalkDir::new(&opt.input) {
        let entry = entry?;
        let input = entry.path();
        if let Some(input_ext) = input.extension() {
            if input_ext == "jpg" || input_ext == "jpeg" || input_ext == "png" {
                let output = &opt
                    .output
                    .join(&input.strip_prefix(&opt.input)?)
                    .with_extension("webp");

                fs::create_dir_all(output.parent().unwrap())?;
                println!("{} -> {}", input.display(), output.display());
                let _ = convert(input, output);
            }
        }
    }

    Ok(())
}

fn convert(input: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let img = image::open(input)?;
    let scale = img.width() as f32 / 720f32;
    let new_height = img.height() as f32 * scale;
    let img = img.resize(720u32, new_height as u32, FilterType::Lanczos3);
    img.save(output)?;
    Ok(())
}
