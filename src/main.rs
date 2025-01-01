use anyhow::Context as _;
use clap::Parser;
use image::ImageReader;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fs;
use std::io::Cursor;
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

#[derive(Debug)]
struct ConvertItem {
    input: PathBuf,
    output: PathBuf,
}

impl ConvertItem {
    fn new(input: PathBuf, output: PathBuf) -> ConvertItem {
        ConvertItem { input, output }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Cli::parse();

    let ci = WalkDir::new(&opt.input)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|path| is_image(path))
        .map(|input| to_item(input, &opt.input, &opt.output))
        .filter_map(|item| item.ok())
        .filter(|item| !item.output.exists())
        .collect::<Vec<_>>();

    // 先に出力先のフォルダは全部作っておく
    for it in &ci {
        fs::create_dir_all(it.output.parent().context("No parent")?)?
    }

    ci.into_par_iter()
        .map(|it| {
            let result = convert(&it.input, &it.output);
            (it, result)
        })
        .for_each(|it| {
            if let Err(e) = &it.1 {
                eprintln!("{} {}", &it.0.input.display(), e);
            }
        });
    Ok(())
}

fn is_image(input: impl AsRef<Path>) -> bool {
    match input.as_ref().extension() {
        Some(ext) => ext == "jpg" || ext == "jpeg" || ext == "png",
        None => false,
    }
}

fn to_item(
    input: impl AsRef<Path>,
    input_prefix: impl AsRef<Path>,
    output_prefix: impl AsRef<Path>,
) -> anyhow::Result<ConvertItem> {
    let output = output_prefix
        .as_ref()
        .join(input.as_ref().strip_prefix(input_prefix)?)
        .with_extension("webp");
    Ok(ConvertItem::new(
        input.as_ref().to_path_buf(),
        output.to_path_buf(),
    ))
}

fn convert(input: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let content = fs::read(&input)?;
    let img = ImageReader::new(Cursor::new(&content))
        .with_guessed_format()?
        .decode()?;
    img.save(output)?;
    Ok(())
}
