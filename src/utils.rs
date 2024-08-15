use rayon::prelude::*;

pub fn cosine_similarity(vec1: &[f32], vec2: &[f32], normalized: bool) -> f32 {
    let dot_product: f32 = vec1
        .par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| a * b)
        .sum();

    if normalized {
        dot_product
    } else {
        let magnitude1: f32 = vec1.par_iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        let magnitude2: f32 = vec2.par_iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        dot_product / (magnitude1 * magnitude2)
    }
}

pub fn euclidean_distance(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

pub fn manhattan_distance(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn pearson_correlation(vec1: &[f32], vec2: &[f32]) -> f32 {
    let mean1 = vec1.par_iter().sum::<f32>() / vec1.len() as f32;
    let mean2 = vec2.par_iter().sum::<f32>() / vec2.len() as f32;

    let numerator: f32 = vec1
        .par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| (a - mean1) * (b - mean2))
        .sum();

    let denom1: f32 = vec1
        .par_iter()
        .map(|a| (a - mean1).powi(2))
        .sum::<f32>()
        .sqrt();

    let denom2: f32 = vec2
        .par_iter()
        .map(|b| (b - mean2).powi(2))
        .sum::<f32>()
        .sqrt();

    numerator / (denom1 * denom2)
}

// angular distance (vec1, vec2, bool)
pub fn angular_distance(vec1: &[f32], vec2: &[f32], normalized: bool) -> f32 {
    let cosine_sim = cosine_similarity(vec1, vec2, normalized);
    cosine_sim.acos() / std::f32::consts::PI
}

pub fn chebyshev_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f32::max)
}

pub fn dot_product_distance(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| a * b)
        .sum()
}

pub fn minkowski_distance(vec1: &[f32], vec2: &[f32], p: i32) -> f32 {
    vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| (a - b).abs().powi(p))
        .sum::<f32>()
        .powf(1.0 / p as f32)
}
