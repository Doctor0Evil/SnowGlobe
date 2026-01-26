#[derive(serde::Serialize)]
pub struct MaterialBlend {
    pub bagasse_frac: f64,
    pub starch_frac: f64,
    pub mineral_frac: f64,
    pub t90_days: f64,
    pub r_tox: f64,
    pub caloric_density: f64,
}

pub struct CyboNode {
    pub lat: f64,
    pub lon: f64,
    pub v_ms: f64,
    pub a_m2: f64,
    pub rho_kgm3: f64,
    pub cp: f64,
}

#[derive(serde::Serialize)]
pub struct EcoProjection {
    pub node_id: String,
    pub ecoimpact_score: f64,
    pub waste_reduced_kg: f64,
    pub energy_kw: f64,
    pub soil_gain: f64,  // Carbon enhancement
    pub water_gain: f64,  // Pollutant reduction
    pub air_gain: f64,  // Emission offset
    pub hex_stamp: String,
}
