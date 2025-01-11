use image::{ColorType, DynamicImage};

pub fn is_color_profile(img: &DynamicImage) -> bool {
    match img.color() {
        ColorType::L8 | ColorType::La8 | ColorType::L16 | ColorType::La16 => false,
        ColorType::Rgb8
        | ColorType::Rgba8
        | ColorType::Rgb16
        | ColorType::Rgba16
        | ColorType::Rgb32F
        | ColorType::Rgba32F => true,
        _ => unreachable!(),
    }
}

pub fn color_pixel_ratio(img: &DynamicImage, color_pixel_threshold: f64) -> f64 {
    let img = img.clone().into_rgb8();
    let mut sum = 0u32;
    let mut n = 0u32;
    let threshold = (256f64 * 256f64 * color_pixel_threshold) as u32;
    for (_, _, pixel) in img.enumerate_pixels() {
        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];
        let v = v(r, g, b);
        let s = s(r, g, b);
        let sv = s as u32 * v as u32;

        if sv > threshold {
            sum += 1;
        }
        n += 1;
    }

    sum as f64 / n as f64
}

fn v(r: u8, g: u8, b: u8) -> u8 {
    max(r, g, b)
}

fn s(r: u8, g: u8, b: u8) -> u8 {
    let v = v(r, g, b);

    if v == 0 {
        0
    } else {
        (255f64 * ((max(r, g, b) as f64 - min(r, g, b) as f64) / max(r, g, b) as f64)) as u8
    }
}

fn max(r: u8, g: u8, b: u8) -> u8 {
    if r > g && r > b {
        r
    } else if g > b && g > r {
        g
    } else {
        b
    }
}

fn min(r: u8, g: u8, b: u8) -> u8 {
    if r < g && r < b {
        r
    } else if g < b && g < r {
        g
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::monochrome::{max, min, s, v};

    #[test]
    fn max_test() {
        assert_eq!(max(1, 2, 3), 3);
        assert_eq!(max(3, 1, 2), 3);
        assert_eq!(max(2, 3, 1), 3);
    }

    #[test]
    fn min_test() {
        assert_eq!(min(1, 2, 3), 1);
        assert_eq!(min(3, 1, 2), 1);
        assert_eq!(min(2, 3, 1), 1);
    }

    #[test]
    fn v_test() {
        assert_eq!(v(100, 150, 200), 200);
    }

    #[test]
    fn s_test() {
        assert_eq!(s(100, 150, 200), 127);
    }
}
