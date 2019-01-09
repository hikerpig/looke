extern crate image;
extern crate palette;

use image::{GenericImage, Pixel};
use palette::Srgb;
use std::env;
use std::path::Path;

mod color;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: looke img1 img2");
        return;
    }

    let path1 = Path::new(&args[1]);
    let path2 = Path::new(&args[2]);

    let img1 = image::open(&path1).unwrap();
    let img2 = image::open(&path2).unwrap();

    let result = compare_imgs(&img1, &img2, 2.0);

    std::process::exit(match result {
        true => 0,
        false => {
            println!(
                "Images {} and {} differs!",
                path1.display(),
                path2.display()
            );
            -1
        }
    });
}

fn compare_imgs<I, P>(img1: &I, img2: &I, tolerance: f32) -> bool
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let px_iter_1 = img1.pixels();

    for (x, y, p1) in px_iter_1 {
        let p2 = img2.get_pixel(x, y);
        let p_result = ciede2000_comparator(&p1, &p2, tolerance);
        if !p_result {
            return false;
        }
    }

    return true;
}

fn are_colors_same<P>(p1: &P, p2: &P) -> bool
where
    P: Pixel<Subpixel = u8>,
{
    let (k1, k2, k3, _) = p1.channels4();
    let (l1, l2, l3, _) = p2.channels4();
    return (k1 == l1) && (k2 == l2) && (k3 == l3);
}

fn ciede2000_comparator<P>(p1: &P, p2: &P, t: f32) -> bool
where
    P: Pixel<Subpixel = u8>,
{
    let (k1, k2, k3, _) = p1.channels4();
    let (l1, l2, l3, _) = p2.channels4();
    if are_colors_same(p1, p2) {
        return true;
    }
    let c1 = color::rgb_to_lab(&Srgb::new(k1 as f32, k2 as f32, k3 as f32));
    let c2 = color::rgb_to_lab(&Srgb::new(l1 as f32, l2 as f32, l3 as f32));

    let d = color::ciede2000_diff(&c1, &c2);
    // println!("dE is {}", d);
    return d < t;
}
