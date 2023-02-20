use image::imageops::FilterType;
use image::{io::Reader as ImageReader, ImageOutputFormat};
use neon::{prelude::*, types::buffer::TypedArray};

fn process_image(
    image_buffer: &[u8],
    max_dimension: u32,
    format: ImageOutputFormat,
) -> Result<Vec<u8>, String> {
    let image = ImageReader::new(std::io::Cursor::new(image_buffer))
        .with_guessed_format()
        .map_err(|e| format!("Failed to decode image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let height = image.height();
    let width = image.width();

    let aspect_ratio = width as f32 / height as f32;
    let (new_width, new_height) = if width > height {
        (max_dimension, (max_dimension as f32 / aspect_ratio) as u32)
    } else {
        ((max_dimension as f32 * aspect_ratio) as u32, max_dimension)
    };

    let resized_image = image.resize(new_width, new_height, FilterType::Nearest);

    let mut compressed_buffer = std::io::Cursor::new(Vec::new());

    resized_image
        .write_to(&mut compressed_buffer, format)
        .map_err(|e| format!("Failed to convert image: {}", e))?;

    Ok(compressed_buffer.into_inner())
}

fn convert(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let max_dimension_arg = cx.argument::<JsNumber>(1)?;
    let max_dimension = max_dimension_arg.value(&mut cx) as u32;

    let format_arg = cx.argument_opt(2);
    let format_string = match format_arg {
        Some(_) => cx.argument::<JsString>(2)?.value(&mut cx),
        None => "webp".to_string(),
    };
    let format = match format_string.as_str() {
        "avif" => ImageOutputFormat::Avif,
        "webp" => ImageOutputFormat::WebP,
        "png" => ImageOutputFormat::Png,
        "jpg" => ImageOutputFormat::Jpeg(75),
        "tiff" => ImageOutputFormat::Tiff,
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        "tga" => ImageOutputFormat::Tga,
        _ => ImageOutputFormat::WebP,
    };

    let buf_arg = cx.argument::<JsBuffer>(0)?;
    let buf = buf_arg.as_slice(&cx);
    match process_image(buf, max_dimension, format) {
        Ok(compressed_buffer) => {
            let mut buffer = cx.buffer(compressed_buffer.len())?;
            buffer
                .as_mut_slice(&mut cx)
                .copy_from_slice(&compressed_buffer);
            Ok(buffer)
        }
        Err(e) => cx.throw_error(e),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("convert", convert)?;
    Ok(())
}
