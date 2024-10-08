
mod tracer;

use tracer::Colour;

fn main() {
    let image_width = 256;
    let image_height = 256; 
    print!("P3\n {} {}\n255\n" , image_width, image_height);

    for i in 0..image_height {
        eprintln!("\rScanlines remaining :{} " , i);
        for j in 0..image_width {
            let pixel_color = Colour {
                x : j as f64/ (image_width-1)  as f64 ,
                y : i as f64 / (image_height-1) as f64 ,
                z : 0.0
            };
            pixel_color.write_colour();
        }
    }
}