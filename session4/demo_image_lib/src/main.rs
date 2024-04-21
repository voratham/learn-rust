use image::io::Reader as ImageReader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let img_original = ImageReader::open("02.png")?.decode()?;

    img_original.thumbnail(100, 100);
    img_original.save("02.jpg")?;

    Ok(())
}
