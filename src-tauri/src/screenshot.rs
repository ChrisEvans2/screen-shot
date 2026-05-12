use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use screenshots::Screen;

pub fn capture_full_screen() -> Result<Vec<u8>, String> {
    let screens = Screen::all().map_err(|e| e.to_string())?;

    if let Some(screen) = screens.first() {
        let image = screen.capture().map_err(|e| e.to_string())?;
        let mut png_data = Vec::new();
        let encoder = PngEncoder::new(&mut png_data);
        encoder
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ColorType::Rgba8,
            )
            .map_err(|e| e.to_string())?;

        Ok(png_data)
    } else {
        Err("No screen found".to_string())
    }
}
