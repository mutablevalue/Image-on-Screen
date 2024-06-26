use image::DynamicImage;
use opener::open;
use reqwest;
use std::path::PathBuf;
use std::sync::Arc;
use tokio;

async fn image_fetch(url: &str) -> Result<Arc<DynamicImage>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.bytes().await?;
    let img = image::load_from_memory(&response)?;
    Ok(Arc::new(img))
}

fn save_image_to_temp_file(
    image: Arc<DynamicImage>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let mut path = temp_dir.path().to_path_buf();
    path.set_file_name("fetched_image.png");

    match image.save(&path) {
        Ok(_) => Ok(path.to_path_buf()),
        Err(e) => {
            eprintln!("Error saving image: {}", e);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to save image",
            )) as Box<dyn std::error::Error>)
        }
    }
}

fn open_image(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    opener::open(path)?;
    Ok(())
}

async fn image_fetch_and_display(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let image = image_fetch(&url).await?;
    match save_image_to_temp_file(image) {
        Ok(temp_path) => open_image(temp_path),
        Err(e) => return Err(e),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <image_url>");
        return;
    }
    match image_fetch_and_display(args[1].clone()).await {
        Ok(_) => println!("Image displayed successfully."),
        Err(e) => eprintln!("Failed to display image: {}", e),
    }
}
