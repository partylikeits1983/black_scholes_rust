pub fn erf(mut x: f64) -> f64 {
    const A1: f64 = 0.254829592;
    const A2: f64 = -0.284496736;
    const A3: f64 = 1.421413741;
    const A4: f64 = -1.453152027;
    const A5: f64 = 1.061405429;
    const P: f64 = 0.3275911;

    let mut sign = 1;
    if x < 0.0 {
        sign = -1;
    }
    x = x.abs();

    let t: f64 = 1.0 / (1.0 + P * x);
    let y: f64 = 1.0 - (((((A5 * t + A4) * t) + A3) * t + A2) * t + A1) * t * (-x * x).exp();

    let result: f64 = sign as f64 * y;

    return result;
}

pub fn cdf(x: f64) -> f64 {
    const SQRT_2: f64 = 1.41421356237;

    let y = (erf(x / SQRT_2) + 1.0) / 2.0;

    return y;
}
