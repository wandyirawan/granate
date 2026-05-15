// Image processing & Minio upload for Granate CMS
// Variants: thumb (200x200), catalog (600x600), full (1200x1200)

use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use image::{ImageFormat, DynamicImage, imageops::FilterType};
use std::io::Cursor;
use uuid::Uuid;

pub const THUMB_SIZE: u32 = 200;
pub const CATALOG_SIZE: u32 = 600;
pub const FULL_SIZE: u32 = 1200;

#[derive(Debug, Clone)]
pub struct ImageVariant {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub size_bytes: u64,
    pub mime_type: String,
}

pub struct ProcessedImage {
    pub original: ImageVariant,
    pub variants: Vec<ImageVariant>,
}

/// Compress & resize image, generate variants
pub fn process_image(data: &[u8], filename: &str) -> Result<ProcessedImage, String> {
    let img = image::load_from_memory(data).map_err(|e| format!("Decode error: {e}"))?;
    let (orig_w, orig_h) = (img.width(), img.height());

    let format = detect_format(filename);
    let mime = format_to_mime(format);

    // Compress original (quality 85)
    let original = encode_image(&img, format, 85)?;

    let mut variants = Vec::new();

    // Thumb: 200x200 center crop
    let thumb = crop_square(&img, THUMB_SIZE);
    let thumb_encoded = encode_image(&thumb, format, 80)?;
    variants.push(ImageVariant {
        name: "thumb".into(),
        width: THUMB_SIZE,
        height: THUMB_SIZE,
        size_bytes: thumb_encoded.len() as u64,
        mime_type: mime.clone(),
        data: thumb_encoded,
    });

    // Catalog: max 600x600
    if orig_w > CATALOG_SIZE || orig_h > CATALOG_SIZE {
        let catalog = img.resize(CATALOG_SIZE, CATALOG_SIZE, FilterType::Lanczos3);
        let cat_encoded = encode_image(&catalog, format, 82)?;
        let (cw, ch) = (catalog.width(), catalog.height());
        variants.push(ImageVariant {
            name: "catalog".into(),
            width: cw,
            height: ch,
            size_bytes: cat_encoded.len() as u64,
            mime_type: mime.clone(),
            data: cat_encoded,
        });
    }

    // Full: max 1200x1200
    if orig_w > FULL_SIZE || orig_h > FULL_SIZE {
        let full = img.resize(FULL_SIZE, FULL_SIZE, FilterType::Lanczos3);
        let full_encoded = encode_image(&full, format, 80)?;
        let (fw, fh) = (full.width(), full.height());
        variants.push(ImageVariant {
            name: "full".into(),
            width: fw,
            height: fh,
            size_bytes: full_encoded.len() as u64,
            mime_type: mime.clone(),
            data: full_encoded,
        });
    }

    Ok(ProcessedImage {
        original: ImageVariant {
            name: "original".into(),
            width: orig_w,
            height: orig_h,
            size_bytes: original.len() as u64,
            mime_type: mime,
            data: original,
        },
        variants,
    })
}

/// Upload processed image + variants to Minio
pub async fn upload_to_minio(
    processed: &ProcessedImage,
    media_id: Uuid,
    filename: &str,
    client: &S3Client,
    bucket_name: &str,
) -> Result<Vec<ImageVariant>, String> {
    let ext = extension(filename);
    let prefix = format!("media/{}/{}", &media_id.to_string()[..8], media_id);
    let mut uploaded = Vec::new();

    // Upload original
    let orig_key = format!("{}/original.{}", prefix, ext);
    client
        .put_object()
        .bucket(bucket_name)
        .key(&orig_key)
        .body(ByteStream::from(processed.original.data.clone()))
        .content_type(&processed.original.mime_type)
        .send()
        .await
        .map_err(|e| format!("Upload original: {e}"))?;
    uploaded.push(ImageVariant {
        name: "original".into(),
        width: processed.original.width,
        height: processed.original.height,
        size_bytes: processed.original.size_bytes,
        mime_type: processed.original.mime_type.clone(),
        data: vec![],
    });

    // Upload variants
    for v in &processed.variants {
        let vkey = format!("{}/{}.{}", prefix, v.name, ext);
        client
            .put_object()
            .bucket(bucket_name)
            .key(&vkey)
            .body(ByteStream::from(v.data.clone()))
            .content_type(&v.mime_type)
            .send()
            .await
            .map_err(|e| format!("Upload {}: {e}", v.name))?;
        uploaded.push(ImageVariant {
            name: v.name.clone(),
            width: v.width,
            height: v.height,
            size_bytes: v.size_bytes,
            mime_type: v.mime_type.clone(),
            data: vec![],
        });
    }

    Ok(uploaded)
}

pub fn build_s3_url(endpoint: &str, bucket: &str, key: &str) -> String {
    let host = endpoint.split(':').next().unwrap_or("localhost");
    format!("http://{}:9000/{}/{}", host, bucket, key)
}

fn detect_format(filename: &str) -> ImageFormat {
    let lower = filename.to_lowercase();
    if lower.ends_with(".png") { return ImageFormat::Png; }
    if lower.ends_with(".webp") { return ImageFormat::WebP; }
    if lower.ends_with(".gif") { return ImageFormat::Gif; }
    ImageFormat::Jpeg
}

fn format_to_mime(f: ImageFormat) -> String {
    match f {
        ImageFormat::Png => "image/png".into(),
        ImageFormat::WebP => "image/webp".into(),
        ImageFormat::Gif => "image/gif".into(),
        _ => "image/jpeg".into(),
    }
}

fn encode_image(img: &DynamicImage, format: ImageFormat, quality: u8) -> Result<Vec<u8>, String> {
    let mut buf = Cursor::new(Vec::new());
    if format == ImageFormat::Jpeg {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
        img.write_with_encoder(encoder).map_err(|e| format!("Encode: {e}"))?;
    } else {
        img.write_to(&mut buf, format).map_err(|e| format!("Encode: {e}"))?;
    }
    Ok(buf.into_inner())
}

fn crop_square(img: &DynamicImage, size: u32) -> DynamicImage {
    let (w, h) = (img.width(), img.height());
    if w == h {
        return img.resize_exact(size, size, FilterType::Lanczos3);
    }
    let min = w.min(h);
    let x = (w - min) / 2;
    let y = (h - min) / 2;
    img.crop_imm(x, y, min, min).resize_exact(size, size, FilterType::Lanczos3)
}

fn extension(filename: &str) -> String {
    filename.split('.').last().unwrap_or("jpg").to_lowercase()
}
