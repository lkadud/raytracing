#[derive(Debug)]
struct vec3(f32, f32, f32);
impl vec3 {
    fn X(&self) -> f32 {
        self.0
    }
    fn Y(&self) -> f32 {
        self.1
    }
    fn Z(&self) -> f32 {
        self.2
    }
    fn length_squared(&self) -> f32 {
        return self.0*self.0 +self.1*self.1 +self.2*self.2
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
}
use vec3 as Point;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\r Scanlines remaining: {} ", image_height-j);
        for i in 0..image_width {
            let r: f32 = i as f32 / (image_width as f32 - 1.0);
            let g: f32 = j as f32 / (image_height as f32 - 1.0);
            let b: f32 = 0.0;

            let ir  = (255.999 * r) as i32;
            let ig  = (255.999 * g) as i32;
            let ib  = (255.999 * b) as i32;

            print!("{ir} {ig} {ib}\n");            
        }
    }
    eprint!("\rDone.                      \n");
}
