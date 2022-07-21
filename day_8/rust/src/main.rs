use std::{env, path, fs, error::Error};

#[derive(Debug)]
struct Layer {
    width: i32,
    height: i32,
    pixels: Vec<usize>,
    digit_counts: [i32; 10]
}

impl Layer {
    fn new(width: i32, height: i32) -> Self {
        let pixels = Vec::new();
        Layer { width, height, pixels, digit_counts: [0; 10] }
    }

    fn add(&mut self, d: char) {
        let digit = d.to_digit(10).unwrap() as usize;
        let count = self.digit_counts[digit];
        self.digit_counts[digit] = count + 1;
        self.pixels.push(digit);
    }

    fn multiply_counts(&self, a: usize, b:usize) -> i32 {
        let a_count = self.digit_counts[a];
        let b_count = self.digit_counts[b];

        a_count * b_count
    }

    fn get_pixel(&self, w: usize, h: usize) -> usize {
        self.pixels[h * self.width as usize + w]
    }
}

#[derive(Debug)]
struct Image {
    width: i32,
    height: i32,
    layers: Vec<Layer>
}

impl Image {
    fn new(width: i32, height: i32, space_image_format: Vec<char>) -> Self {
        let mut layers: Vec<Layer> = Vec::new();
        let layer_count = space_image_format.len() as i32 / (width * height);
        for l in 0..layer_count {
            let mut layer = Layer::new(width, height);
            for d in l * height * width..(l + 1) * height * width {
                layer.add(space_image_format[d as usize]);
            }
            layers.push(layer);
        }
        Image { height, width, layers }
    }

    fn decode_image(&self) -> Vec<Vec<usize>> {
        let mut image = vec![vec![2; self.width as usize]; self.height as usize];
        for l in &self.layers {
            for h in 0..self.height as usize {
                for w in 0..self.width as usize {
                    let current = image[h][w];
                    if current < 2 {
                        continue;
                    }
                    image[h][w] = l.get_pixel(w, h);
                }
            }
        }

        image
    }

    fn get_fewest_digit_layer(&self, digit: usize) -> usize {
        let mut smallest_layer = 0;
        let mut smallest = i32::max_value();

        for (i, l) in self.layers.iter().enumerate() {
            let count = l.digit_counts[digit];
            if count < smallest {
                smallest = count;
                smallest_layer = i;
            }
        }
        smallest_layer
    }

    fn multiply_counts(&self, l: usize, a: usize, b:usize) -> i32 {
        self.layers[l].multiply_counts(a, b)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sif : Vec<char> = get_file().unwrap();
    let i = Image::new(25, 6, sif);

    let l = i.get_fewest_digit_layer(0);

    println!("{:?}", l);

    let mc = i.multiply_counts(l, 1, 2);

    println!("{:?}", mc);

    let image = i.decode_image();

    for l in image {
        println!("{:?}", String::from(l.iter().map(|element| match element { 1 => "#", 0 => " ", _ => ""}).collect::<String>()));
    }

    Ok(())
}

pub fn get_file() -> Option<Vec<char>> {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = &args.get(1).expect("Supply path param");
    println!("{}", path);

    let path = path::Path::new(path);
    if !path.exists() {
        println!("Path unreachable");
        return None;
    }

    let file = fs::read_to_string(path).ok()?;

    let code : Vec<char> = file.trim().chars().collect();
    
    Some(code)
}

#[cfg(test)]
mod test;
