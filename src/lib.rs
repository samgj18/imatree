use anyhow::{Context, Error};
use clap::Parser;
use image::{ImageBuffer, Rgb};
use std::path::{Path, PathBuf};
use text_to_png::TextRenderer;

/// Simple program to create background images with text
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The name of the image to create
    #[arg(short, long)]
    name: String,

    /// Path to save the image
    #[arg(short, long, default_value = ".")]
    path: String,

    /// The color of the text
    #[arg(short, long, default_value = "Black")]
    color: String,

    /// The font size of the text
    #[arg(short, long, default_value_t = 0)]
    font_size: u32,

    /// The width of the image
    #[arg(short, long, default_value_t = 1000)]
    width: u32,

    /// The height of the image
    #[arg(short = 't', long, default_value_t = 1000)]
    height: u32,
}

pub trait Renderer {
    fn render_text_to_png_data(&self) -> Result<(), Error>;
}

impl Renderer for Args {
    fn render_text_to_png_data(&self) -> Result<(), Error> {
        // Sanitize the name
        let file_name: String = sanitize_name(&self.name);

        // Create a white background image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            create_white_background_image(self.width, self.height);

        // Write the text in the middle of the image
        let renderer: TextRenderer = TextRenderer::default();

        // Dinamically choose the font size based on the text length
        let font_size: u32 = determine_font_size(
            &self.name,
            if self.font_size == 0 {
                None
            } else {
                Some(self.font_size)
            },
        );

        // Render the text to png data
        let mut image: Image = generate_background_image(
            renderer,
            &self.name,
            &self.color,
            font_size,
            PathBuf::from(&self.path).join(&file_name),
            img,
        )?;

        // Overlay the text on the image at the center
        let image = generate_image_overlay(&mut image, self.width, self.height);

        // Save the image as "file_name.png"
        image
            .img
            .save(Path::new(&self.path).join(&file_name))
            .context("Failed to save image")?;

        Ok(())
    }
}

struct Image {
    overlay: image::DynamicImage,
    img: image::DynamicImage,
}

fn sanitize_name(name: &str) -> String {
    name.trim()
        .replace(' ', "_")
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c == &'_')
        .collect::<String>()
        + ".png"
}

fn create_white_background_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([255_u8, 255, 255_u8]);
    }
    img
}

fn determine_font_size(name: &str, font_size: Option<u32>) -> u32 {
    let len = name.len();

    font_size.unwrap_or(match len {
        0..=10 => 100,
        11..=20 => 75,
        21..=30 => 50,
        31..=40 => 25,
        41..=50 => 17,
        51..=60 => 10,
        _ => 5,
    })
}

fn generate_background_image(
    renderer: TextRenderer,
    name: &str,
    color: &str,
    font_size: u32,
    path: PathBuf,
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> Result<Image, Error> {
    let png = renderer
        .render_text_to_png_data(name, font_size, color)
        .context("Failed to render text to png data")?;

    img.save(&path).context("Failed to save image")?;
    let new_image = image::open(path).context("Failed to open image")?;

    let overlay = image::load_from_memory(&png.data).context("Failed to load image")?;

    Ok(Image {
        overlay,
        img: new_image,
    })
}

fn generate_image_overlay(image: &mut Image, width: u32, height: u32) -> &mut Image {
    image::imageops::overlay(
        &mut image.img,
        &image.overlay,
        // place the text in the middle of the image
        ((width - image.overlay.width()) / 2).into(),
        ((height - image.overlay.height()) / 2).into(),
    );

    image
}
