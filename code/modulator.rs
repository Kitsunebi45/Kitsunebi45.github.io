// Vixen Cybernetics: Bio-Modulator
// File: src/biological/modulator.rs

pub struct BioModulator {
    pub estrogen_level: f64,    // 0.0 to 1.0 (Normalized)
    pub sleep_quality: f64,     // 0.0 to 1.0
    pub stress_cortisol: f64,   // 0.0 to 1.0
}

impl BioModulator {
    /// Calculates the current "Signal Gain" for Pattern Recognition.
    /// Higher Estrogen + Low Stress = Maximum Resolution (High Intuition).
    pub fn calculate_signal_gain(&self) -> f64 {
        // Estrogen boosts signal detection (Research verified)
        let signal_boost = 1.0 + (self.estrogen_level * 0.5);
        
        // Cortisol increases noise (Anxiety/Paranoia)
        let noise_dampening = 1.0 - (self.stress_cortisol * 0.8);
        
        signal_boost * noise_dampening
    }
}
