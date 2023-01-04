mod black_scholes;

fn main() {
    println!("Calculating Black Scholes");

    let option = black_scholes::BS {price: 100.0, strike: 80.0, expiry: 0.5, rate: 0.1, sigma: 0.9};

    let call = black_scholes::bs_call(&option);
    println!("Call Option Price: {}", call);

    let put = black_scholes::bs_put(&option);
    println!("Put Option Price: {}", put);
}
