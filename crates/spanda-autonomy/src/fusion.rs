//! Sensory fusion and confidence scoring.
//!
use serde::{Deserialize, Serialize};

/// Confidence for a single sensor or signal source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorConfidence {
    pub source: String,
    pub value: f64,
    pub confidence: f64,
    pub timestamp: Option<String>,
}

/// Policy for minimum confidence and conflict handling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfidencePolicy {
    pub min_confidence: f64,
    pub require_agreement: bool,
    pub conflict_threshold: f64,
}

impl Default for ConfidencePolicy {
    fn default() -> Self {
        Self {
            min_confidence: 0.6,
            require_agreement: true,
            conflict_threshold: 0.25,
        }
    }
}

/// Agreement between multiple sensor readings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalAgreement {
    pub sources: Vec<String>,
    pub agreement_score: f64,
}

/// Detected conflict between sensor signals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalConflict {
    pub sources: Vec<String>,
    pub delta: f64,
    pub description: String,
}

/// Fused observation from multiple sources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FusedObservation {
    pub metric: String,
    pub fused_value: f64,
    pub confidence: ConfidenceScore,
    pub agreement: Option<SignalAgreement>,
    pub conflicts: Vec<SignalConflict>,
}

/// Overall confidence score for a fused observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceScore {
    pub score: f64,
    pub meets_policy: bool,
    pub recommendation: String,
}

/// Fuse observations using weighted average and detect conflicts.
pub fn fuse_observations(
    metric: &str,
    readings: &[SensorConfidence],
    policy: &ConfidencePolicy,
) -> FusedObservation {
    if readings.is_empty() {
        return FusedObservation {
            metric: metric.into(),
            fused_value: 0.0,
            confidence: ConfidenceScore {
                score: 0.0,
                meets_policy: false,
                recommendation: "no_sources".into(),
            },
            agreement: None,
            conflicts: vec![],
        };
    }

    let total_weight: f64 = readings.iter().map(|r| r.confidence.max(0.01)).sum();
    let fused_value = readings
        .iter()
        .map(|r| r.value * r.confidence.max(0.01))
        .sum::<f64>()
        / total_weight;
    let avg_confidence = readings.iter().map(|r| r.confidence).sum::<f64>() / readings.len() as f64;
    let conflicts = detect_signal_conflict(readings, policy.conflict_threshold);
    let agreement = if conflicts.is_empty() && readings.len() > 1 {
        Some(SignalAgreement {
            sources: readings.iter().map(|r| r.source.clone()).collect(),
            agreement_score: 1.0 - conflicts.len() as f64 * 0.1,
        })
    } else {
        None
    };
    let meets = avg_confidence >= policy.min_confidence && conflicts.is_empty();
    let recommendation = if !meets && !conflicts.is_empty() {
        "lower_readiness_and_diagnose".into()
    } else if !meets {
        "require_fallback".into()
    } else {
        "proceed".into()
    };
    FusedObservation {
        metric: metric.into(),
        fused_value,
        confidence: ConfidenceScore {
            score: avg_confidence,
            meets_policy: meets,
            recommendation,
        },
        agreement,
        conflicts,
    }
}

/// Detect pairwise conflicts when normalized delta exceeds threshold.
pub fn detect_signal_conflict(
    readings: &[SensorConfidence],
    threshold: f64,
) -> Vec<SignalConflict> {
    let mut conflicts = Vec::new();
    for i in 0..readings.len() {
        for j in (i + 1)..readings.len() {
            let a = &readings[i];
            let b = &readings[j];
            let denom = a.value.abs().max(b.value.abs()).max(1.0);
            let delta = (a.value - b.value).abs() / denom;
            if delta > threshold {
                conflicts.push(SignalConflict {
                    sources: vec![a.source.clone(), b.source.clone()],
                    delta,
                    description: format!(
                        "{} vs {} delta {:.2} exceeds {:.2}",
                        a.source, b.source, delta, threshold
                    ),
                });
            }
        }
    }
    conflicts
}
