extern crate winapi;
extern crate winapi_util;

use image::{GenericImageView, ImageBuffer, Rgb};
use std::io::{Error, ErrorKind};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsString;

use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER};
use winapi::shared::minwindef::BOOL;

fn set_wallpaper_from_image(image_path: &str) -> Result<(), std::io::Error> {
    let img = image::open(image_path).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    println!("[debug] Opened source file");

    let (width, height) = img.dimensions();
    println!("[debug] Recovered image dimensions");

    let mut buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    println!("[debug] Loaded image buffer");

    for (x, y, pixel) in img.pixels() {
        buffer.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
    }

    let temp_path: OsString = std::env::temp_dir().into();
    println!("[debug] Retrieved TEMP folder location");

    let image_name = format!("{}.jpg", std::path::Path::new(image_path).file_stem().unwrap().to_string_lossy());
    println!("[debug] Transitioned the image file");

    let image_path = temp_path.clone().into_string().unwrap() + "\\" + &image_name;

    buffer.save_with_format(&image_path, image::ImageFormat::Jpeg).expect("[error] Error at Heat from Fire");
    println!("[debug] Saved image at TEMP");

    let path: Vec<u16> = OsString::from(image_path).encode_wide().chain(std::iter::once(0)).collect();
    println!("[debug] Formatted path");

    let result: BOOL = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut _,
            0x01 | 0x02,
        )
    };

    println!("[debug] Background set command sent");

    match result {
        0 => {
            println!("[Error] Error occurred at Skies of November");
            Err(Error::last_os_error())
        }
        _ => {
            println!("[debug] Desktop wallpaper set successfully!");
            Ok(())
        }
    }
}

fn main() {
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();
    print!("Enter the path of the image: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let path = input.trim();
    match set_wallpaper_from_image(path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error setting desktop wallpaper: {}", e),
    }
}