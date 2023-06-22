use std::fs::File;
use std::io::{ErrorKind, Write};

pub mod lal;

fn write_to(fileName : &str, bytes : &[u8]) -> std::io::Result<()>
{
    let mut ppm_file = File::open(fileName).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound{
            File::create(fileName).unwrap_or_else(|error| {
                panic!("cannot create file {:?}", error);
            })
        }
        else {
            panic!("cannot opening file with other problems {:?}", error);
        }
    });

    ppm_file.write_all(bytes);
    ppm_file.flush()?;
    Ok(())
}

//The pixels are written out in rows with pixels left to right.
//The rows are written out from top to bottom.
fn create_pixel_buffer(width : u32, height : u32) -> String
{
    let mut temp = String::new();
    temp.push_str("P3\n");
    temp.push_str(&width.to_string());
    temp.push_str(" ");
    temp.push_str(&height.to_string());
    temp.push_str("\n255\n");

    for y in 0..height{
        for x in 0..width{
            // let r : f32 = x as f32 / width as f32;
            // let g : f32 = (height - y) as f32 / height as f32;
            // let b : f32 = 0.0;
            let col = lal::vec3::create(
                x as f64 / width as f64, 
                (height - y) as f64 / height as f64,
                0.0);

            let ir = (255.99 * col.r() ) as i32;
            let ig = (255.99 * col.g() ) as i32;
            let ib = (255.99 * col.b() ) as i32;
            //println!("{} {} {}", ir, ig, ib);
            temp.push_str(&ir.to_string());
            temp.push_str(" ");
            temp.push_str(&ig.to_string());
            temp.push_str(" ");
            temp.push_str(&ib.to_string());
            temp.push_str("\n");

        }
    }
    temp
}

fn main() {
    let num_x = 200;
    let num_y = 100;

    // let mut test_aaa = lal::vec3::create(11.11, 30.5, 11.11);

    // let test_bbb = lal::vec3::create(23.11, 34.656, 24.989);

    // let unit_vector_test = lal::vec3::unit_vector(test_aaa);

    // println!("cross result is {:#?}", unit_vector_test);



    //println!("{}", result);
    let ppm_context = create_pixel_buffer(num_x, num_y);
    write_to("hello_test.ppm", ppm_context.as_bytes()).expect("error------->");
}
