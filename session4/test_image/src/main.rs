use std::io::Cursor;

use thumbnailer::{create_thumbnails, ThumbnailSize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Wait to process image!");
    let file = std::fs::File::open("test.png").unwrap();
    let reader = std::io::BufReader::new(file);

    let size = (100, 100);
    let mut thumbnails =
        create_thumbnails(reader, mime::IMAGE_PNG, [ThumbnailSize::Custom(size)]).unwrap();

    let thumbnail = thumbnails.pop().unwrap();

    let mut buf = Cursor::new(Vec::new());

    thumbnail.write_jpeg(&mut buf, 100)?;
    std::fs::write("test_convert.jpg", buf.into_inner())?;

    Ok(())
}
