// Vixen Cybernetics // The Foundry
// Module: Project Tiamat (Collapse Forecaster)
// Context: Metacybernetic Risk Assessment (The Institute)

use serde::{Deserialize, Serialize};

/// The Input Vector.
/// Represents the current metacybernetic coordinates of the target system.
///
/// NOTE: For Personal Topology (Micro-Scale):
/// - energy_substrate = Physical spoons/ATP/Sleep debt.
/// - wealth_pump_velocity = Dopamine extraction vs. Replenishment.
/// - narrative_coherence = Internal Monologue Stability (Self-Concept).
/// - mass_mobilization = Anxiety/Somatic arousal levels.
/// - elite_mobilization = Executive Function/Superego rigidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    /// The physical flow (Oil/Grid/ATP). 0.0 = Total Blackout.
    pub energy_substrate: f64,
    
    /// Inequality/Extraction rate. High (1.0) = Panic accumulation.
    pub wealth_pump_velocity: f64,
    
    /// Is the story holding? Low (0.0) = Bašmu (Toxic Bloom).
    pub narrative_coherence: f64,
    
    /// The TI_D variable. Observer Detachment.
    /// High = Clear Signal. Low = Bias/Panic.
    pub observer_detachment: f64,
    
    /// Hunger/Poverty/Somatic Scream levels (MMP).
    pub mass_mobilization: f64,
    
    /// Internal betrayal risk/Executive Dysfunction (EMP).
    pub elite_mobilization: f64,
}

/// The Shadow Archetypes.
/// These are the attractors the system falls into when stability breaks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShadowArchetype {
    /// Ugallu (The Storm): Hard Crash.
    /// Panic, looting, immediate resource scarcity.
    Ugallu,
    
    /// Uridimmu (The Mad Dog): Civil War.
    /// High kinetic violence, purges, neighbor-vs-neighbor.
    Uridimmu,
    
    /// Bašmu (The Toxic Bloom): Viral Disinfo.
    /// Reality fracture, cult generation, mass psychosis.
    Basmu,
    
    /// Girtablullu (The Gatekeeper): Totalitarian Lockdown.
    /// Military law, checkpoints, elite ring-fencing.
    Girtablullu,
    
    /// Vixen Integration (The Phoenix).
    /// Successful transition through the fire.
    VixenIntegration,
    
    /// System appears stable (or metastabile).
    Stable,
}

/// A single discrete record of a simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub input_state: SystemState,
    pub result: Result<ShadowArchetype, String>,
}

/// The BlackBox Recorder.
/// Maintains the longitudinal history of the system's trajectory.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlackBoxRecorder {
    pub history: Vec<LogEntry>,
}

impl BlackBoxRecorder {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    /// Runs a simulation and logs the result automatically.
    pub fn capture_snapshot(&mut self, timestamp: &str, state: SystemState) -> Result<ShadowArchetype, String> {
        let prediction = TiamatEngine::calculate_risk(&state);
        
        self.history.push(LogEntry {
            timestamp: timestamp.to_string(),
            input_state: state,
            // We clone the result to store it. 
            // In a real DB, we might store the error string differently.
            result: prediction.clone(),
        });

        prediction
    }

    /// Returns the last calculated risk state.
    pub fn last_known_state(&self) -> Option<&Result<ShadowArchetype, String>> {
        self.history.last().map(|entry| &entry.result)
    }
}

/// The Logic Engine.
pub struct TiamatEngine;

impl TiamatEngine {
    /// Calculates the Probability of System Collapse and maps it to an Archetype.
    /// 
    /// # Returns
    /// * `Ok(ShadowArchetype)` - The predicted attractor state.
    /// * `Err(String)` - Bias Error if the Observer is compromised.
    pub fn calculate_risk(state: &SystemState) -> Result<ShadowArchetype, String> {
        // --- GATE 1: BIAS CHECK (The TI_D Variable) ---
        // The system refuses to predict if you are panicking.
        // If the observer cannot separate from the event, the data is noise.
        if state.observer_detachment < 0.1 {
            return Err("Bias Error: Observer too close to Event. Increase TI_D parameters.".to_string());
        }

        // --- GATE 2: HARD CRASH CHECK (Physics) ---
        // If the energy substrate fails, sociology doesn't matter. Biology takes over.
        // Condition: Energy < 0.2
        if state.energy_substrate < 0.2 {
            // Output: Ugallu (Panic/Looting)
            return Ok(ShadowArchetype::Ugallu);
        }

        // --- GATE 3: REVOLUTION CHECK (Societal) ---
        // If wealth is siphoning up (Velocity > 0.9) AND the justification story fails (Coherence < 0.3).
        // The "Social Contract" snaps.
        if state.wealth_pump_velocity > 0.9 && state.narrative_coherence < 0.3 {
            // Output: Uridimmu (Civil War/The Mad Dog)
            return Ok(ShadowArchetype::Uridimmu);
        }

        // --- GATE 4: TOTALITARIAN CHECK (The Gatekeeper) ---
        // If Elites are highly mobilized (betrayal risk/fear) and Wealth Pump is high.
        // They will lock the gates to protect the assets.
        if state.elite_mobilization > 0.8 && state.wealth_pump_velocity > 0.7 {
            return Ok(ShadowArchetype::Girtablullu);
        }

        // --- GATE 5: TOXIC BLOOM CHECK (The Bašmu) ---
        // If Narrative is dead (< 0.2) but Mass Mobilization is High (> 0.7).
        // The energy has nowhere to go but into insanity/conspiracy.
        if state.narrative_coherence < 0.2 && state.mass_mobilization > 0.7 {
            return Ok(ShadowArchetype::Basmu);
        }

        // --- GATE 6: THE PHOENIX (Vixen Integration) ---
        // High Observer Detachment (Lucidity), Moderate Narrative (Honesty),
        // and sufficient Energy to rebuild.
        if state.observer_detachment > 0.8 && state.narrative_coherence > 0.5 && state.energy_substrate > 0.4 {
            return Ok(ShadowArchetype::VixenIntegration);
        }

        // If no collapse vectors are critical:
        Ok(ShadowArchetype::Stable)
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_log() {
        let mut recorder = BlackBoxRecorder::new();
        
        let state = SystemState {
            energy_substrate: 0.15, // Hard crash
            wealth_pump_velocity: 0.5,
            narrative_coherence: 0.5,
            observer_detachment: 0.9,
            mass_mobilization: 0.2,
            elite_mobilization: 0.2,
        };

        let result = recorder.capture_snapshot("2025-01-01T12:00:00Z", state);
        
        assert_eq!(result, Ok(ShadowArchetype::Ugallu));
        assert_eq!(recorder.history.len(), 1);
        assert_eq!(recorder.history[0].timestamp, "2025-01-01T12:00:00Z");
    }

    #[test]
    fn test_bias_gate_lockout() {
        let panic_state = SystemState {
            energy_substrate: 0.5,
            wealth_pump_velocity: 0.5,
            narrative_coherence: 0.5,
            observer_detachment: 0.05, // Too low
            mass_mobilization: 0.5,
            elite_mobilization: 0.5,
        };
        
        match TiamatEngine::calculate_risk(&panic_state) {
            Err(e) => assert!(e.contains("Bias Error")),
            Ok(_) => panic!("Should have failed bias check"),
        }
    }
}