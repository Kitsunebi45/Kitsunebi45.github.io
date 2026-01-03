// Vixen Cybernetics: The Safety Net
// File: src/game_theory/safety_net.rs

pub struct SafetyNet {
    pub target_node_id: String, // Ace
    pub engagement_mode: EngagementMode,
}

pub enum EngagementMode {
    ActiveControl,      // Trying to stop him (Bad for Rebellion)
    WitnessSupport,     // "Peanut Gallery" (Good for Trust)
    EmergencyCatch,     // Life-saving intervention only
}

impl SafetyNet {
    /// The "Wink" Logic:
    /// If Target is destabilizing BUT not critical, switch to WitnessSupport.
    /// This prevents "Reactance" (Psychological push-back).
    pub fn update_strategy(&mut self, target_stability: f64) {
        if target_stability < 0.15 {
            self.engagement_mode = EngagementMode::EmergencyCatch;
        } else if target_stability < 0.50 {
            // He is wobbling, but let him wobble.
            // "I know he would catch me."
            self.engagement_mode = EngagementMode::WitnessSupport; 
        } else {
            self.engagement_mode = EngagementMode::ActiveControl; // Standard interaction
        }
    }
}
