extern crate image;
extern crate palette;

#[macro_use]
extern crate clap;

mod diff;
mod color;

use clap::{App, Arg};
use std::path::Path;

use diff::*;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("ref-image")
                .help("path to reference image")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("image")
                .help("path to image")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("tolerance")
                .help("tolerance for image diff")
                .short("t")
                .long("tolerance")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("diff-image")
                .short("d")
                .long("diff-image")
                .takes_value(true)
                .help("path for saving diff output image"),
        );

    let matches = app.get_matches();
    let ref_image = matches.value_of("ref-image").unwrap();
    let image = matches.value_of("image").unwrap();
    let diff_image_v = matches.value_of("diff-image");
    let tolerance: f32 = matches
        .value_of("tolerance")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0.0);

    let path1 = Path::new(&ref_image);
    let path2 = Path::new(&image);

    let img1 = image::open(&path1).unwrap();
    let img2 = image::open(&path2).unwrap();

    let result = compare_imgs(&img1, &img2, tolerance);

    let comparator: PixelComparator =
        Box::new(move |p1, p2| ciede2000_compare(p1, p2, tolerance));

    match result.equal {
        true => {
            std::process::exit(0);
        }
        false => {
            println!(
                "Images {} and {} differs!",
                path1.display(),
                path2.display()
            );
            println!("Diff area {:?}", result.diff_area);
            let img_buffer = build_diff_image(&img1, &img2, comparator);

            match diff_image_v {
                Some(v) => {
                    let p = Path::new(v);
                    img_buffer.save(p).unwrap();
                    println!("diff image saved to {}", v)
                }
                None => {}
            }

            std::process::exit(1);
        }
    };
}


