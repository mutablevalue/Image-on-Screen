use image::{DynamicImage, ImageOutputFormat};
use opener::open_blocking;
use reqwest;
use std::error::Error;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use tokio;

async fn image_fetch(url: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let response = reqwest::get(url).await?.bytes().await?;
    let img = image::load_from_memory(&response)?;
    Ok(img)
}

fn save_image_to_temp_file(image: DynamicImage) -> Result<NamedTempFile, Box<dyn Error>> {
    let mut temp_file = NamedTempFile::new()?;
    image.write_to(&mut temp_file, ImageOutputFormat::Png)?;
    Ok(temp_file)
}

fn open_image(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    open_blocking(path)?;
    Ok(())
}

async fn image_fetch_and_display(url: String) -> Result<(), Box<dyn Error>> {
    let image = image_fetch(&url).await?;
    let temp_file = save_image_to_temp_file(image)?;
    let temp_path = temp_file.path().to_path_buf();

    open_image(&temp_path)?;

    // Keep temp_file alive until here to prevent deletion
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <image_url>");
        return;
    }
    if let Err(e) = image_fetch_and_display(args[1].clone()).await {
        eprintln!("Failed to display image: {}", e);
    }
}
