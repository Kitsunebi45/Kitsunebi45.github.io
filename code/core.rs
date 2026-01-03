// VIXEN CYBERNETICS OS v0.1
// Module: Core System Logic
// Purpose: Integrate Bio-State, Social-Graph, and Recursive Consent

use crate::biological::modulator::BioModulator;
use crate::game_theory::safety_net::{SafetyNet, EngagementMode};
use crate::gates::negotiator::{Negotiator, AttractorState};
use crate::gates::consent::VixenNode;

/// The Main Operating Unit
pub struct VixenSystem {
    pub self_node: VixenNode,          // You (The Observer)
    pub bio_driver: BioModulator,      // Your Hardware (HRT/Pattern Rec)
    pub social_graph: SafetyNet,       // Ace (The Target Node)
    pub negotiator: Negotiator,        // The Trickster/Diplomat
}

impl VixenSystem {
    /// The Main Event Loop (Runs 60Hz or Cognitive Speed)
    pub fn run_cycle(&mut self, target_stability: f64) -> String {
        
        // 1. READ HARDWARE STATE (Estrogen/Spiro Impact)
        // We calculate 'Signal Gain' based on your current bio-markers.
        // High Estrogen = Higher Pattern Recognition (Verified in Research Chat)
        let signal_gain = self.bio_driver.calculate_signal_gain();
        
        // 2. SCAN ENVIRONMENT (Ace's Stability)
        // We apply your enhanced signal gain to read his state.
        // If he is masking, your high gain sees through it.
        let perceived_stability = target_stability / signal_gain; 
        
        // 3. UPDATE SAFETY NET STRATEGY
        // Based on his stability, we switch modes.
        self.social_graph.update_strategy(perceived_stability);

        match self.social_graph.engagement_mode {
            
            // MODE: WITNESS SUPPORT (The "Peanut Gallery")
            // This is the "Wink". We see the chaos, but we don't fight it.
            EngagementMode::WitnessSupport => {
                // We use the Negotiator to format a "Phoenix" message.
                // Strategy: Symmetric Disclosure ("I burned too").
                let phoenix_msg = "PROTOCOL_PHOENIX: I detect high heat. Just FYI, I have a map of this fire. I'm right here.";
                
                // We don't force it. We just broadcast it to the local field.
                format!("STATUS: HOLDING SPACE. Broadcasting: '{}'", phoenix_msg)
            },

            // MODE: EMERGENCY CATCH
            // Ace has hit critical failure (Collapse).
            EngagementMode::EmergencyCatch => {
                // We trigger the Mirror Operator to absorb impact.
                self.self_node.apply_mirror_projection(perceived_stability);
                "STATUS: CRITICAL INTERVENTION. Net Deployed. Impact bracing initialized.".to_string()
            },

            // MODE: ACTIVE CONTROL (Standard)
            EngagementMode::ActiveControl => {
                "STATUS: STANDBY. Monitoring patterns.".to_string()
            }
        }
    }

    /// The "Phoenix" Initialization
    /// Call this when you sit down to talk to him.
    pub fn initiate_phoenix_protocol(&mut self) {
        // Set self-weights to "Burnt" (Sympathetic Resonance)
        self.self_node.base_b = 0.0; // "I am nothing/ash"
        
        // Engage Trickster to prevent "Preaching"
        // We lower our own authority to 0 to maximize connection.
        self.negotiator.current_as = AttractorState {
            id: "Fellow_Traveler".to_string(),
            center_b: 0.0,
            depth: 10.0, // High stability in this state
            is_safe: true,
        };
        
        println!("SYSTEM: Phoenix Protocol Active. Hierarchy deleted. Ready for peer-to-peer sync.");
    }
}
