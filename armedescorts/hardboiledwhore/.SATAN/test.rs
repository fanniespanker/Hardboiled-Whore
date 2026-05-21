use std::f32;

const TAU_BASE: f32 = 1.0;

// ---------- CONFIG ----------
const ALPHA_SMALL: f32 = 1.2;
const BETA_MED: f32 = 0.8;
const GAMMA_LARGE: f32 = 0.6;

// ---------- ENTRY ----------
fn main() {
    let text = "The box was empty. Except for the breathing.";

    let tokens = tokenize(text);

    let mut x: Vec<f32> = Vec::new();        // interest signal
    let mut delta_tau: Vec<f32> = Vec::new();
    let mut curvature: Vec<f32> = Vec::new();

    // fake semantic trajectory
    let mut prev_vec = random_vec();
    let mut prev_v = vec![0.0; prev_vec.len()];

    for (t, token) in tokens.iter().enumerate() {
        // 1. naive interest
        let interest = interest_of(token);
        x.push(interest);

        // 2. wavelets (2-level Haar)
        let (small, medium, large) = wavelet_bands(&x);

        // 3. Δτ update
        let dt = update_tau(small, medium, large);
        delta_tau.push(dt);

        // 4. fake semantic movement
        let curr_vec = perturb_vec(&prev_vec, interest);

        // velocity
        let v = diff(&curr_vec, &prev_vec);

        // acceleration
        let a = diff(&v, &prev_v);

        let kappa = norm(&a);
        curvature.push(kappa);

        // feedback into interest (very simple)
        let adjusted = interest + 0.2 * kappa;
        x[t] = adjusted;

        // log
        println!(
            "{:02} | {:<12} | x={:.3} small={:.3} med={:.3} large={:.3} Δτ={:.3} κ={:.3}",
            t, token, x[t], small, medium, large, dt, kappa
        );

        prev_vec = curr_vec;
        prev_v = v;
    }
}

// ---------- TOKENIZATION ----------
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// ---------- INTEREST (DUMB HEURISTIC) ----------
fn interest_of(token: &str) -> f32 {
    let len = token.len() as f32;

    let punctuation_boost = if token.ends_with('.') { 0.3 } else { 0.0 };

    (len / 10.0).min(1.0) + punctuation_boost
}

// ---------- WAVELETS ----------
fn wavelet_bands(xs: &Vec<f32>) -> (f32, f32, f32) {
    if xs.len() < 2 {
        return (0.0, 0.0, 0.0);
    }

    // Level 1 (small)
    let mut diffs_lvl1 = Vec::new();
    for i in (0..xs.len() - 1).step_by(2) {
        let d = (xs[i] - xs[i + 1]).abs();
        diffs_lvl1.push(d);
    }

    let small = l2(&diffs_lvl1);

    // Level 2 (medium)
    let mut avgs = Vec::new();
    for i in (0..xs.len() - 1).step_by(2) {
        avgs.push((xs[i] + xs[i + 1]) * 0.5);
    }

    let mut diffs_lvl2 = Vec::new();
    for i in (0..avgs.len() - 1).step_by(2) {
        let d = (avgs[i] - avgs[i + 1]).abs();
        diffs_lvl2.push(d);
    }

    let medium = l2(&diffs_lvl2);

    // Large = overall variance proxy
    let large = variance(xs);

    (small, medium, large)
}

// ---------- Δτ ----------
fn update_tau(small: f32, medium: f32, large: f32) -> f32 {
    let zoom = 1.0 + ALPHA_SMALL * small;
    let hold = 1.0 + BETA_MED * medium;
    let flow = 1.0 + GAMMA_LARGE * large;

    let tau = TAU_BASE * zoom * hold / flow;

    tau.clamp(0.5, 3.0)
}

// ---------- VECTOR UTILS ----------
fn random_vec() -> Vec<f32> {
    (0..8).map(|_| rand()).collect()
}

fn perturb_vec(v: &Vec<f32>, scale: f32) -> Vec<f32> {
    v.iter().map(|x| x + scale * (rand() - 0.5)).collect()
}

fn diff(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}

fn norm(v: &Vec<f32>) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}

// ---------- STATS ----------
fn l2(xs: &Vec<f32>) -> f32 {
    norm(xs)
}

fn variance(xs: &Vec<f32>) -> f32 {
    if xs.is_empty() {
        return 0.0;
    }
    let mean = xs.iter().sum::<f32>() / xs.len() as f32;
    xs.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / xs.len() as f32
}

// ---------- RNG ----------
fn rand() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    (nanos % 1000) as f32 / 1000.0
}