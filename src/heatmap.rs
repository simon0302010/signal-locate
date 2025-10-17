use image::{RgbImage, Rgb};
use rand::Rng;

// function to create test heatmap
pub fn generate_random() {
    let points_count = 10;
    let mut points: Vec<(usize, usize, f64)> = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..points_count {
        let x = rng.random_range(0..1920);
        let y = rng.random_range(0..1080);
        let strength = rng.random_range(0.0..100.0);
        points.push((x, y, strength));
    }

    let img = gen_heatmap(&points, 1920, 1080, 200);
    img.save("heatmap.png").unwrap();
}

pub fn gen_heatmap(points: &[(usize, usize, f64)], width: usize, height: usize, radius: usize) -> RgbImage {
    let sigma = radius as f64 / 2.0;
    let mut heatmap = vec![vec![0.0f64; width]; height];

    for &(px, py, strength) in points {
        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - px as i32;
                let dy = y as i32 - py as i32;
                let dist2 = dx*dx + dy*dy;
                let weight = (-dist2 as f64 / (2.0 * sigma * sigma)).exp();
                heatmap[y][x] += strength * weight;
            }
        }
    }

    let max = heatmap.iter().flat_map(|row| row.iter()).cloned().fold(0./0., f64::max);
    let mut img = RgbImage::new(width as u32, height as u32);
    for (y, row) in heatmap.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let norm = (val / max).min(1.0);
            let color = heatmap_color(norm);
            img.put_pixel(x as u32, y as u32, color);
        }
    }
    img
}

fn heatmap_color(v: f64) -> Rgb<u8> {
    let v = v.max(0.0).min(1.0);
    let r = (255.0 * (v - 0.5).max(0.0) * 2.0) as u8;
    let g = (255.0 * (1.0 - (v - 0.5).abs() * 2.0)) as u8;
    let b = (255.0 * (0.5 - v).max(0.0) * 2.0) as u8;
    Rgb([r, g, b])
}