// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

extern crate clap;
use clap::{command, Parser, Subcommand};
use crate::Commands::Fractal;

#[derive(Parser)]
#[command(name="mirage", version="0.1.0", author="mrbarge")]
struct Cli {
    #[arg(short, long)]
    infile: Option<String>,
    #[arg(short, long)]
    outfile: String,
    #[command(subcommand)]
    command: Commands,
}

struct CropRange {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(clap::ValueEnum, Clone, Copy)]
enum RotateDegrees {
    R90,
    R180,
    R270
}

#[derive(Subcommand)]
enum Commands {
    Blur {
        #[arg(short, long, default_value_t=2.0)]
        val: f32,
    },
    Brighten {
        #[arg(short, long, default_value_t=10)]
        val: i32,
    },
    Crop {
        #[arg(short, long)]
        x: u32,
        #[arg(short, long)]
        y: u32,
        #[arg(long)]
        width: u32,
        #[arg(long)]
        height: u32,
    },
    Rotate {
        #[arg(short, long, value_enum)]
        degrees: RotateDegrees,
    },
    Generate {
        #[arg(short, long)]
        r: u8,
        #[arg(short, long)]
        g: u8,
        #[arg(short, long)]
        b: u8,
    },
    Invert,
    Grayscale,
    Fractal,
}


fn main() {
    let cli = Cli::parse();
    if cli.infile.is_none() {
        match &cli.command {
            Commands::Fractal => {},
            Commands::Generate{r,g,b} => {},
            _ => {
                println!("Error: infile parameter required");
                std::process::exit(1);
            }
        }
    }
    match &cli.command {
        Commands::Blur { val } => {
            blur(cli.infile.unwrap(), cli.outfile, *val);
        },
        Commands::Brighten { val } => {
            brighten(cli.infile.unwrap(), cli.outfile, *val);
        },
        Commands::Grayscale => {
            grayscale(cli.infile.unwrap(), cli.outfile);
        },
        Commands::Crop { x, y, width, height } => {
            let cr = CropRange{x: *x, y: *y, w: *width, h: *height };
            crop(cli.infile.unwrap(), cli.outfile, cr);
        },
        Commands::Invert => {
            invert(cli.infile.unwrap(), cli.outfile);
        },
        Commands::Rotate { degrees } => {
            rotate(cli.infile.unwrap(), cli.outfile, *degrees);
        },
        Commands::Generate { r, g, b } => {
            generate(cli.outfile, *r, *g, *b);
        }
        Commands::Fractal => {
            fractal(cli.outfile);
        }
    }
}

fn blur(infile: String, outfile: String, v: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(v);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, v: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(v);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, cr: CropRange) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.crop(cr.x, cr.y, cr.w, cr.h);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, degrees: RotateDegrees) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = match degrees {
        RotateDegrees::R90 => img.rotate90(),
        RotateDegrees::R180 => img.rotate180(),
        RotateDegrees::R270 => img.rotate270(),
    };
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.grayscale();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, r: u8, g: u8, b: u8) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    let (mut mr, mut mg, mut mb): (u8, u8, u8) = (r, g, b);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Do some weird gradient thing I guess
        *pixel = image::Rgb([mr, mg, mb]);
        mr = (mr % 255) + 1;
        mg = (mg % 255) + 1;
        mb = (mb % 255) + 1;
    }
    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
