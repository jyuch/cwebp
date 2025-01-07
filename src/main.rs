use clap::Parser;
use image::imageops::FilterType;
use image::{ColorType, DynamicImage, GenericImageView, ImageReader};
use indicatif::{ProgressBar, ProgressStyle};
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

    /// Image extension
    #[clap(short, long, default_value = "avif")]
    ext: String,

    /// Print help.
    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
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
        .map(|input| to_item(input, &opt.input, &opt.output, &opt.ext))
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

    let total = ci.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("#>-"),
    );

    for it in &ci {
        let result = convert(&it.input, &it.output, opt.width, opt.height);
        if let Err(e) = result {
            eprintln!("{} {}", &it.input.display(), e);
        }
        pb.inc(1);
    }

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
    output_extension: &str,
) -> anyhow::Result<ConvertItem> {
    let output = output_prefix
        .as_ref()
        .join(input.as_ref().strip_prefix(input_prefix)?)
        .with_extension(output_extension);
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

    let img: DynamicImage = match img.color() {
        ColorType::L8 | ColorType::La8 | ColorType::L16 | ColorType::La16 => {
            DynamicImage::from(img.into_luma8())
        }
        ColorType::Rgb8
        | ColorType::Rgba8
        | ColorType::Rgb16
        | ColorType::Rgba16
        | ColorType::Rgb32F
        | ColorType::Rgba32F => DynamicImage::from(img.into_rgb8()),
        _ => unreachable!(),
    };

    img.save(output)?;
    Ok(())
}
