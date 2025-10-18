use image::{RgbImage, Rgb};
use colorgrad::{CatmullRomGradient, GradientBuilder, Gradient};
use rand::Rng;

// function to create test heatmap
#[allow(dead_code)]
pub fn generate_random() {
    let points_count = 20;
    let mut points: Vec<(f64, f64, f64)> = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..points_count {
        let x = rng.random_range(0..1920) as f64;
        let y = rng.random_range(0..1080) as f64;
        let strength = rng.random_range(0.0..100.0);
        points.push((x, y, strength));
    }

    let img = gen_heatmap(&points, 1920, 1080, 200);
    img.save("heatmap.png").unwrap();
}

// point: (x, y, strength)
pub fn gen_heatmap(points: &[(f64, f64, f64)], width: usize, height: usize, radius: usize) -> RgbImage {
    let grad = GradientBuilder::new()
            .html_colors(&["red", "yellow", "green"])
            .build::<colorgrad::CatmullRomGradient>()
            .unwrap();

    let sigma = radius as f64 / 2.0;
    let mut heatmap = vec![vec![0.0f64; width]; height];

    for &(px, py, strength) in points {
        let px = px as i32;
        let py = py as i32;
        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - px;
                let dy = y as i32 - py;
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
            let color = heatmap_color(norm, &grad);
            img.put_pixel(x as u32, y as u32, color);
        }
    }
    img
}

fn heatmap_color(v: f64, grad: &CatmullRomGradient) -> Rgb<u8> {
    let v = v.max(0.0).min(1.0);
    let col = grad.at(v as f32);
    Rgb([
        (col.r * 255.0) as u8,
        (col.g * 255.0) as u8,
        (col.b * 255.0) as u8,
    ])
}