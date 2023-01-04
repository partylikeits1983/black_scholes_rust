mod stats;

pub struct BS {
    pub price: f64,
    pub strike: f64,
    pub expiry: f64,
    pub rate: f64,
    pub sigma: f64
}

pub fn d1(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let d1: f64 = (s/k).ln() + (r + sigma.powf(2.0) / 2.0) / (sigma * t.sqrt());
    return d1;
}

pub fn d2(d1: f64, sigma: f64, t: f64) -> f64 {
    let d2: f64 = d1 - sigma * t.sqrt();
    return d2;
}

pub fn bs_call(option: &BS) -> f64 {
    let s: f64 = option.price;
    let k: f64 = option.strike;
    let t: f64 = option.expiry;
    let r: f64 = option.rate;
    let sigma: f64 = option.sigma;

    let d1: f64 = d1(s, k, t, r, sigma);
    let d2: f64 = d2(d1, sigma, t);

    let c = s * stats::cdf(d1) - k * (-r * t).exp() * stats::cdf(d2);

    return c;
}

pub fn bs_put(option: &BS) -> f64 {
    let s: f64 = option.price;
    let k: f64 = option.strike;
    let t: f64 = option.expiry;
    let r: f64 = option.rate;
    let sigma: f64 = option.sigma;

    let d1: f64 = d1(s, k, t, r, sigma);
    let d2: f64 = d2(d1, sigma, t);

    let c = k * (-r*t).exp() * stats::cdf(-d2) - s * stats::cdf(-d1);

    return c;
}
