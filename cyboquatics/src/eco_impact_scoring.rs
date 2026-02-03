use std::f64::consts::PI;

#[derive(Debug)]
struct CyboquaticParams {
    density: f64,  // kg/m³
    area: f64,     // m²
    velocity: f64, // m/s
    cp: f64,       // power coefficient
    removal_eff: f64, // fraction
}

fn calculate_power(params: &CyboquaticParams) -> f64 {
    0.5 * params.density * params.area * params.velocity.powi(3) * params.cp * 0.80
}

fn eco_impact(power: f64, eff: f64, time: f64) -> f64 {
    power * eff * time  // time-weighted impact in watts-hours-equivalent
}

fn main() {
    let params = CyboquaticParams {
        density: 1025.0,
        area: PI * (2.0 / 2.0).powi(2),  // example rotor
        velocity: 2.0,
        cp: 0.38,
        removal_eff: 0.94,
    };
    let power = calculate_power(&params);
    let impact = eco_impact(power, params.removal_eff, 8760.0);  // annual hours
    println!("Eco-Impact: {:.2}", impact);
}
