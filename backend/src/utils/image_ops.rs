use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use image::GenericImage;

pub fn apply_watermark(
    mut base: DynamicImage,
    watermark: &DynamicImage,
    opacity: f32,
) -> DynamicImage {
    let (bw, bh) = base.dimensions();
    let (ww, wh) = watermark.dimensions();

    let x = bw - ww - 10;
    let y = bh - wh - 10;

    for i in 0..ww {
        for j in 0..wh {
            let mut wp = watermark.get_pixel(i, j);
            let mut bp = base.get_pixel(x + i, y + j);

            wp.0[3] = ((wp.0[3] as f32) * opacity) as u8;

            let alpha = wp.0[3] as f32 / 255.0;

            bp.0[0] = (bp.0[0] as f32 * (1.0 - alpha) + wp.0[0] as f32 * alpha) as u8;
            bp.0[1] = (bp.0[1] as f32 * (1.0 - alpha) + wp.0[1] as f32 * alpha) as u8;
            bp.0[2] = (bp.0[2] as f32 * (1.0 - alpha) + wp.0[2] as f32 * alpha) as u8;

            base.put_pixel(x + i, y + j, bp);
        }
    }

    base
}
