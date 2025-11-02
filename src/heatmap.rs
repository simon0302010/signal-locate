use colorgrad::{CatmullRomGradient, Gradient, GradientBuilder};
use image::{Rgb, RgbImage};
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
pub fn gen_heatmap(
    points: &[(f64, f64, f64)],
    width: usize,
    height: usize,
    radius: usize,
) -> RgbImage {
    let grad = GradientBuilder::new()
        .html_colors(&["red", "yellow", "green"])
        .build::<colorgrad::CatmullRomGradient>()
        .unwrap();

    let sigma = radius as f64 / 2.0;
    let mut heatmap = vec![vec![0.0f64; width]; height];

    for &(px, py, strength) in points {
        let px_int = px as i32;
        let py_int = py as i32;

        let r = (radius * 3) as i32;
        let y_min = (py_int - r).max(0) as usize;
        let y_max = (py_int + r).min(height as i32 - 1) as usize;
        let x_min = (px_int - r).max(0) as usize;
        let x_max = (px_int + r).min(width as i32 - 1) as usize;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let dx = x as f64 - px;
                let dy = y as f64 - py;
                let dist2 = dx * dx + dy * dy;
                let weight = (-dist2 / (2.0 * sigma * sigma)).exp();
                heatmap[y][x] += strength * weight;
            }
        }
    }

    let max = heatmap
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let max = if max > 0.0 { max } else { 1.0 };

    let mut img = RgbImage::new(width as u32, height as u32);

    for (y, row) in heatmap.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let norm = (val / max).clamp(0.0, 1.0);
            let color = heatmap_color(norm, &grad);
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img
}

fn heatmap_color(v: f64, grad: &CatmullRomGradient) -> Rgb<u8> {
    let col = grad.at(v.clamp(0.0, 1.0) as f32);
    Rgb([
        (col.r * 255.0) as u8,
        (col.g * 255.0) as u8,
        (col.b * 255.0) as u8,
    ])
}
