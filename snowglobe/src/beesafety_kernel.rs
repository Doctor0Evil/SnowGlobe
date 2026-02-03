#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorridorKind {
    EMF,
    Thermal,
    Acoustic,
    Chemical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorEnvelope {
    pub kind: CorridorKind,
    pub l_min: f32,
    pub l_max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeeContext {
    pub sensitivity: f32,
    pub in_hive_exclusion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub node_id: u32,
    pub duty_cycle: f32,
    pub mass_removed: f32,
    pub nano_karma: f32,
    pub power_normalized: f32,
    pub bee_ctx: BeeContext,
    pub predicted_levels: [f32; 4],
    pub r_sigma: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelParams {
    pub eta_mass: f32,
    pub eta_karma: f32,
    pub eta_bee: f32,
    pub m_ref: f32,
    pub phi_ref: f32,
    pub r_sigma_max: f32,
}

#[derive(Debug)]
pub enum KernelError {
    EmptyCorridors,
    InvalidDutyCycle,
    MismatchedModalities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelDecision {
    pub safe_duty_cycle: f32,
    pub permitted: bool,
    pub phi_penalty: f32,
    pub eco_impact_bee: f32,
    pub v_bee: f32,
}

pub struct BeeSafetyKernel {
    corridors: [CorridorEnvelope; 4],
    params: KernelParams,
}

impl BeeSafetyKernel {
    pub fn new(corridors: [CorridorEnvelope; 4], params: KernelParams) -> Result<Self, KernelError> {
        if corridors.len() != 4 {
            return Err(KernelError::EmptyCorridors);
        }
        Ok(Self { corridors, params })
    }

    fn compute_phi(&self, state: &NodeState) -> f32 {
        let mut phi = 0.0;
        for (i, &level) in state.predicted_levels.iter().enumerate() {
            let env = &self.corridors[i];
            let d_over = (level - env.l_max).max(0.0);
            let d_under = (env.l_min - level).max(0.0);
            phi += d_over * d_over + d_under * d_under;
        }
        if state.bee_ctx.in_hive_exclusion {
            phi *= 1e6;
        }
        phi
    }

    fn compute_v_bee(&self, state: &NodeState, phi: f32) -> f32 {
        let mut v = phi / self.params.phi_ref;
        for &sigma in &state.r_sigma {
            v += sigma / self.params.r_sigma_max;
        }
        v
    }

    pub fn evaluate_node(&self, state: NodeState) -> Result<KernelDecision, KernelError> {
        if !(0.0..=1.0).contains(&state.duty_cycle) {
            return Err(KernelError::InvalidDutyCycle);
        }
        let phi = self.compute_phi(&state);
        let v_bee = self.compute_v_bee(&state, phi);
        let m_norm = state.mass_removed / self.params.m_ref;
        let k_norm = state.nano_karma;
        let u_new = state.duty_cycle + self.params.eta_mass * m_norm + self.params.eta_karma * k_norm - self.params.eta_bee * (phi / self.params.phi_ref);
        let safe_duty = u_new.clamp(0.0, 1.0);
        let permitted = phi == 0.0 && !state.bee_ctx.in_hive_exclusion && v_bee <= 1.0;
        let eco_impact_bee = 1.0 - v_bee;
        Ok(KernelDecision { safe_duty_cycle: safe_duty, permitted, phi_penalty: phi, eco_impact_bee, v_bee })
    }
}
