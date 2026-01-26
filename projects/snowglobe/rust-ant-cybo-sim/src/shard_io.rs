use std::path::Path;
use csv::Writer;
use crate::models::EcoProjection;

pub fn write_shard_csv(path: &Path, proj: &mut EcoProjection, hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    proj.hex_stamp = hex.to_string();  // Anchor with bostrom DID
    let mut wtr = Writer::from_path(path)?;
    wtr.serialize(proj)?;
    wtr.flush()?;
    Ok(())
}
