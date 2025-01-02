use clap::Parser;
use image::imageops::FilterType;
use image::{GenericImageView, ImageReader};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
struct Cli {
    /// Input directory.
    #[clap(short, long)]
    input: PathBuf,

    /// Output directory.
    #[clap(short, long)]
    output: PathBuf,

    /// Image width.
    #[clap(short, long)]
    width: Option<u32>,

    /// Image height
    #[clap(short, long)]
    height: Option<u32>,
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
        if let Some(parent) = it.output.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
    }

    ci.into_par_iter()
        .map(|it| {
            let result = convert(&it.input, &it.output, opt.width, opt.height);
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

fn convert(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    width: Option<u32>,
    height: Option<u32>,
) -> anyhow::Result<()> {
    let content = fs::read(&input)?;
    let img = ImageReader::new(Cursor::new(&content))
        .with_guessed_format()?
        .decode()?;

    let (cur_width, cur_height) = img.dimensions();
    let new_width = width.unwrap_or(cur_width);
    let new_height = height.unwrap_or(cur_height);
    let img = img.resize(new_width, new_height, FilterType::Lanczos3);
    img.save(output)?;
    Ok(())
}
