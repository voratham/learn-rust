use anyhow::Ok;
use image::io::Reader as ImageReader;

use image::{DynamicImage, ImageFormat};

// NOTE: before refactor
// async fn make_image_thumb(img_path: &str) -> anyhow::Result<()> {
//     let file_name_splited: Vec<&str> = img_path.rsplitn(2, ".").collect();

//     let out_img_path = format!("{}-thumb.{}", file_name_splited[1], "jpg");

//     let img_original_to_bytes = std::fs::read(img_path)?;

//     let guess = image::guess_format(&img_original_to_bytes)?;

//     if guess == ImageFormat::Png {
//         let img_reader = ImageReader::open(img_path)?;

//         let img = img_reader.decode()?;
//         if img.as_rgba8().is_some() {
//             let img_converted = img.to_rgb8();
//             let result: DynamicImage = DynamicImage::ImageRgb8(img_converted);
//             let thumb_img = result.thumbnail(100, 100);
//             thumb_img.save(out_img_path.clone())?;
//         } else {
//             let thumb_img = img.thumbnail(100, 100);
//             thumb_img.save(out_img_path.clone())?;
//         }
//     }

//     if guess == ImageFormat::Jpeg {
//         let img = image::load_from_memory_with_format(&img_original_to_bytes, guess)?;
//         let thumb_img = img.thumbnail(100, 100);
//         thumb_img.save(out_img_path.clone())?;
//     }
//     Ok(())
// }

async fn make_image_thumb(img_path: &str) -> anyhow::Result<()> {
    let file_name_splited: Vec<&str> = img_path.rsplitn(2, ".").collect();

    let out_img_path = format!("{}-thumb.{}", file_name_splited[1], "jpg");

    let img_original_to_bytes = std::fs::read(img_path)?;

    let guess = image::guess_format(&img_original_to_bytes)?;

    let img = match guess {
        ImageFormat::Png => {
            let img_reader = ImageReader::open(img_path)?;
            let img = img_reader.decode()?;

            if img.as_rgba8().is_some() {
                let img: DynamicImage = DynamicImage::ImageRgb8(img.to_rgb8());
                img
            } else {
                img
            }
        }
        ImageFormat::Jpeg => image::load_from_memory_with_format(&img_original_to_bytes, guess)?,
        _ => anyhow::bail!("Unsupported image format: {:?}", guess),
    };

    let thumb_img = img.thumbnail(100, 100);
    thumb_img.save(out_img_path)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 start program...");
    let images: Vec<&str> = vec![
        "02.png",
        "blue2.jpg",
        "green.jpg",
        "mock2.jpg",
        "mock3.jpeg",
        "blue.png",
    ];

    for img_element in images {
        make_image_thumb(img_element).await?;
    }

    println!("🟢 finished");

    Ok(())
}
