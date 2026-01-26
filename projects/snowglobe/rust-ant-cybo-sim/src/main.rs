use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::models::{MaterialBlend, CyboNode, EcoProjection};
use crate::sim_kernel::{simulate_biodeg, compute_hydro_power, project_eco_gains};
use crate::shard_io::write_shard_csv;

mod models;
mod sim_kernel;
mod shard_io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define ant-safe blend: 70% bagasse, 25% starch, 5% mineral; caloric density 0.25
    let blend = MaterialBlend {
        bagasse_frac: 0.70,
        starch_frac: 0.25,
        mineral_frac: 0.05,
        t90_days: 85.0,
        r_tox: 0.08,
        caloric_density: 0.25,
    };

    // Define Phoenix cyboquatic node: Central-AZ canal segment
    let node = CyboNode {
        lat: 33.45,
        lon: -112.07,
        v_ms: 2.0,  // Flow velocity
        a_m2: 2.0,  // Turbine area
        rho_kgm3: 1000.0,
        cp: 0.4,
    };

    // Simulate biodegradation (Monod kinetics)
    let biodeg_res = simulate_biodeg(&blend);

    // Compute hydro power
    let power_kw = compute_hydro_power(&node);

    // Project eco-gains
    let projection = project_eco_gains(&blend, &node, biodeg_res, power_kw);

    // Output shard with hex-stamp
    let shard_path = Path::new("qpudatashards/particles/AntCyboSimPhoenix2026v1.csv");
    std::fs::create_dir_all(shard_path.parent().unwrap())?;
    write_shard_csv(shard_path, &projection, "0x30cc44dd55ee66ff77889900aa11bb22")?;  // Bostrom DID hex-stamp

    Ok(())
}
