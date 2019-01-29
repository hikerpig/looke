use image::{GenericImage, ImageBuffer, Pixel, Rgb};
use palette::Srgb;

use crate::color::{rgb_to_lab, ciede2000_diff};

#[derive(Debug)]
pub struct DiffArea {
    xmin: u32,
    ymin: u32,
    xmax: u32,
    ymax: u32,
}

pub struct DiffResult {
    pub diff_area: DiffArea,
    pub equal: bool,
}

pub fn compare_imgs<I, P>(img1: &I, img2: &I, tolerance: f32) -> DiffResult
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let px_iter_1 = img1.pixels();

    let mut diff_pixels: Vec<(u32, u32)> = vec![];

    for (x, y, p1) in px_iter_1 {
        let p2 = img2.get_pixel(x, y);
        let p_result = ciede2000_compare(p1.channels4(), p2.channels4(), tolerance);
        if !p_result {
            diff_pixels.push((x, y));
        }
    }

    let equal = diff_pixels.is_empty();

    let diff_area = get_diff_area(diff_pixels);

    return DiffResult { equal, diff_area };
}

pub type RgbaTuple = (u8, u8, u8, u8);

pub type PixelComparator = Box<Fn(RgbaTuple, RgbaTuple) -> bool>;

pub fn build_diff_image<I, P>(img1: &I, img2: &I, comparator: PixelComparator) -> image::RgbImage
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();
    let width = w1.max(w2);
    let height = h1.max(h2);
    let min_width = w1.min(w2);
    let min_height = h1.min(h2);
    let mut img_buffer: image::RgbImage = ImageBuffer::new(width, height);
    let highlight_color: Rgb<u8> = Rgb { data: [200, 1, 1] };

    let px_iter_1 = img1.pixels();

    for (x, y, p1) in px_iter_1 {
        let p2 = img2.get_pixel(x, y);
        if x >= min_width || y >= min_height {
            img_buffer.put_pixel(x, y, highlight_color);
            continue;
        }
        let p_result = comparator(p1.channels4(), p2.channels4());
        img_buffer.put_pixel(
            x,
            y,
            if p_result {
                p1.to_rgb()
            } else {
                highlight_color
            },
        );
    }

    img_buffer
}

pub fn get_diff_area(pixels: Vec<(u32, u32)>) -> DiffArea {
    let mut xmax = 0;
    let mut xmin: u32 = std::u32::MAX;
    let mut ymax = 0;
    let mut ymin: u32 = std::u32::MAX;
    pixels.iter().for_each(|t| {
        let x = (*t).0;
        let y = (*t).1;
        if x < xmin {
            xmin = x
        };
        if x > xmax {
            xmax = x
        };
        if y < ymin {
            ymin = y
        };
        if y > ymax {
            ymax = y
        };
    });
    DiffArea {
        xmin,
        ymin,
        xmax,
        ymax,
    }
}

pub fn are_colors_same(p1: RgbaTuple, p2: RgbaTuple) -> bool {
    let (k1, k2, k3, _) = p1;
    let (l1, l2, l3, _) = p2;
    return (k1 == l1) && (k2 == l2) && (k3 == l3);
}

pub fn ciede2000_compare(p1: RgbaTuple, p2: RgbaTuple, t: f32) -> bool {
    let (k1, k2, k3, _) = p1;
    let (l1, l2, l3, _) = p2;
    if are_colors_same(p1, p2) {
        return true;
    }
    let c1 = rgb_to_lab(&Srgb::new(k1 as f32, k2 as f32, k3 as f32));
    let c2 = rgb_to_lab(&Srgb::new(l1 as f32, l2 as f32, l3 as f32));

    let d = ciede2000_diff(&c1, &c2);
    // println!("dE is {}", d);
    return d < t;
}
