use std::fs::File;
use std::io::{ErrorKind, Write};

pub mod lal;
pub mod ray;


enum Hit<T>{
    Yes(T),
    No,
}


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
// p(t) = A + t*B.
//t*t*dot(B,B) + 2*t*dot(A-C,A-C) + dot(C,C) - R*R = 0
fn hit_sphere(center : lal::vector::vec3, radius : f64, r : &ray::ray_basic) -> Hit<f64>
{
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    //println!("{}", discriminant);
    if discriminant < 0.0{
        //-1.0
        Hit::No
    }
    else {
        //println!("hello");
        Hit::Yes((-b - discriminant.sqrt()) / (2.0 * a))
    }
}


fn color(r : &ray::ray_basic) -> lal::vector::vec3{
    let t = hit_sphere(lal::vector::vec3::create(0.0, 0.0, -1.0), 0.5, r);

    match t {
        Hit::Yes(v) => {
            let N = lal::vector::vec3::unit_vector(r.point_at_parameter(v) - lal::vector::vec3::create(0.0,0.0,-1.0));
            0.5 * lal::vector::vec3::create(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0)
        },
        Hit::No =>{
            let unit_direction = lal::vector::vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * lal::vector::vec3::create(1.0, 1.0, 1.0) + t * lal::vector::vec3::create(0.5, 0.7, 1.0)
        }
    }


}

// fn color(r : ray::ray_basic) -> lal::vector::vec3{
//     let unit_direction = lal::vector::vec3::unit_vector(r.direction());
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     lal::vector::vec3::create(1.0, 1.0, 1.0) * (1.0 - t) + lal::vector::vec3::create(0.5, 0.7, 1.0) * t
// }


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

    let lower_left_corner = lal::vector::vec3::create(-2.0, -1.0, -1.0);
    let horizontal = lal::vector::vec3::create(4.0, 0.0, 0.0);
    let vertical = lal::vector::vec3::create(0.0, 2.0, 0.0);
    let origin = lal::vector::vec3::create(0.0, 0.0, 0.0);

    for y in 0..height{
        for x in 0..width{
            // let r : f32 = x as f32 / width as f32;
            // let g : f32 = (height - y) as f32 / height as f32;
            // let b : f32 = 0.0;
            // let col = lal::vector::vec3::create(
            //     x as f64 / width as f64, 
            //     (height - y) as f64 / height as f64,
            //     0.0);



            let u =  x as f64 / width as f64;
            let v = (height - y) as f64 / height as f64;


            let ray_temp =  ray::ray_basic::create(
                origin,
                lower_left_corner + u * horizontal + v * vertical
            );

            // let color_result = lal::vector::vec3::create(
            //     x as f64 / width as f64, 
            //     (height - y) as f64 / height as f64,
            //     0.0);

            let color_result = color(&ray_temp);

            let ir = (255.99 * color_result.r() ) as i32;
            let ig = (255.99 * color_result.g() ) as i32;
            let ib = (255.99 * color_result.b() ) as i32;
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

    // let test_bbb = lal::vector::vec3::create(23.11, 34.656, 24.989);

    // let unit_vector_test = lal::vector::vec3::unit_vector(test_aaa);

    // println!("cross result is {:#?}", unit_vector_test);




    //println!("{}", result);
    let ppm_context = create_pixel_buffer(num_x, num_y);
    write_to("hello_test.ppm", ppm_context.as_bytes()).expect("error------->");
}
