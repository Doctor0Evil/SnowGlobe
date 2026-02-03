use std::f64::consts::PI;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ResearchData {
    knowledge_factor: f64,
    eco_impact: f64,
    risk_harm: f64,
    evidence: String,
}

#[derive(Debug)]
struct SnowGlobePipeline {
    data_store: HashMap<String, ResearchData>,
}

impl SnowGlobePipeline {
    fn new() -> Self {
        SnowGlobePipeline {
            data_store: HashMap::new(),
        }
    }

    fn ingest(&mut self, topic: &str, kf: f64, ei: f64, rh: f64, ev: &str) {
        let data = ResearchData {
            knowledge_factor: kf.max(0.0).min(1.0),
            eco_impact: ei.max(0.0).min(1.0),
            risk_harm: rh.max(0.0).min(0.2),
            evidence: ev.to_string(),
        };
        self.data_store.insert(topic.to_string(), data);
    }

    fn compute_impact(&self, topic: &str, time: f64) -> Option<f64> {
        self.data_store.get(topic).map(|d| {
            let base = d.eco_impact * time;
            base * (1.0 - d.risk_harm) * d.knowledge_factor
        })
    }

    fn hex_stamp(&self, topic: &str) -> String {
        if let Some(d) = self.data_store.get(topic) {
            format!("0x{:x}", (d.eco_impact * 1000.0) as u64)
        } else {
            String::from("0x00000000")
        }
    }
}

fn main() {
    let mut pipeline = SnowGlobePipeline::new();
    pipeline.ingest("WetBulbThresholds", 0.95, 0.97, 0.10, "CTmax=48.3°C bounded by ALN");
    let impact = pipeline.compute_impact("WetBulbThresholds", 8760.0).unwrap_or(0.0);
    let stamp = pipeline.hex_stamp("WetBulbThresholds");
    println!("Time-Weighted Impact: {:.2} | Hex-Stamp: {}", impact, stamp);
}
