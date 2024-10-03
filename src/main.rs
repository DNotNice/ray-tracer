
mod tracer;

use tracer::Vec3;
use std::fs::File;
use std::io::Write;

fn main()->std::io::Result<()> {
    let image_width = 256;
    let image_height = 256; 
    println!("P3\n {}  {} \n" ,  image_width , image_height);
    
    let mut file = File::create("image.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    for i in 0..image_height {
        for j in 0..image_width {   
            let r = j as f64/ (image_width-1)  as f64 ;
            let g = i as f64 / (image_height-1) as f64 ;
            let b = 0.0;

            let ir = (255.999 * r) as i64 ;
            let ig = (255.999 * g) as i64 ;
            let ib = (255.999 * b) as i64 ;

            writeln!(file ,"{} {}  {}" , ir ,ig, ib)?; 
        }
    }
    println!("image is generated ");
    Ok(())
}