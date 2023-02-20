use image::imageops::FilterType;
use image::{io::Reader as ImageReader, ImageOutputFormat};
use neon::{prelude::*, types::buffer::TypedArray};

fn process_image(
    image_buffer: &[u8],
    max_dimension: u32,
    format: ImageOutputFormat,
    filter_type: FilterType,
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

    let resized_image = image.resize(new_width, new_height, filter_type);

    let mut compressed_buffer = std::io::Cursor::new(Vec::new());

    resized_image
        .write_to(&mut compressed_buffer, format)
        .map_err(|e| format!("Failed to convert image: {}", e))?;

    Ok(compressed_buffer.into_inner())
}

fn convert(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let convert_config = cx.argument::<JsObject>(0)?;
    let max_dimension_arg: Handle<JsNumber> = convert_config.get(&mut cx, "max_dimensions")?;
    let max_dimension = max_dimension_arg.value(&mut cx) as u32;

    let format_arg: Result<Option<Handle<JsString>>, neon::result::Throw> =
        convert_config.get_opt(&mut cx, "format");
    let format_string = match format_arg {
        Ok(Some(format_arg)) => format_arg.value(&mut cx),
        _ => "webp".to_string(),
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

    let filter_type_arg: Result<Option<Handle<JsString>>, neon::result::Throw> =
        convert_config.get_opt(&mut cx, "filter");
    let filter_type_string = match filter_type_arg {
        Ok(Some(filter_type_arg)) => filter_type_arg.value(&mut cx),
        _ => "nearest".to_string(),
    };

    let filter_type = match filter_type_string.as_str() {
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        "nearest" => FilterType::Nearest,
        _ => FilterType::Nearest,
    };

    let buf_arg: Handle<JsBuffer> = convert_config.get(&mut cx, "buffer")?;
    let buf = buf_arg.as_slice(&cx);
    match process_image(buf, max_dimension, format, filter_type) {
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
