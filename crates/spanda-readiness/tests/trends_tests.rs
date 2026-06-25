//! Tests for readiness history and trend analysis.

use spanda_readiness::{
    analyze_readiness_trends, load_readiness_history, parse_forecast_horizon,
    record_readiness_snapshot, save_readiness_history, ReadinessFactorScore, ReadinessHistory,
    ReadinessHistoryEntry, ReadinessPolicy, ReadinessReport, ReadinessScore, ReadinessStatus,
};
use std::path::PathBuf;

fn sample_report(score: u32) -> ReadinessReport {
    ReadinessReport {
        status: ReadinessStatus::Ready,
        mission_ready: score >= 80,
        score: ReadinessScore {
            total: score,
            maximum: 100,
            factors: vec![ReadinessFactorScore {
                factor: "hardware".into(),
                score,
                weight: 20,
                weighted: score as f64 * 0.2,
            }],
        },
        issues: Vec::new(),
        policy: ReadinessPolicy::default(),
        target: Some("RoverV1".into()),
        robots: vec!["Rover".into()],
    }
}

#[test]
fn records_and_loads_history_entries() {
    let dir = std::env::temp_dir().join(format!("spanda-readiness-history-{}", std::process::id()));
    let path = dir.join("readiness-history.json");
    let _ = std::fs::remove_dir_all(&dir);

    record_readiness_snapshot(&sample_report(75), "rover.sd", &path).unwrap();
    record_readiness_snapshot(&sample_report(82), "rover.sd", &path).unwrap();

    let history = load_readiness_history(&path);
    assert_eq!(history.entries.len(), 2);
    assert_eq!(history.entries[0].total_score, 75);
    assert_eq!(history.entries[1].total_score, 82);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn analyzes_upward_trend_and_forecast() {
    let mut history = ReadinessHistory {
        version: 1,
        entries: vec![
            ReadinessHistoryEntry {
                recorded_at: "2026-06-01T00:00:00Z".into(),
                program: "rover.sd".into(),
                mission_ready: false,
                total_score: 70,
                maximum_score: 100,
                factors: vec![ReadinessFactorScore {
                    factor: "hardware".into(),
                    score: 70,
                    weight: 20,
                    weighted: 14.0,
                }],
            },
            ReadinessHistoryEntry {
                recorded_at: "2026-06-08T00:00:00Z".into(),
                program: "rover.sd".into(),
                mission_ready: true,
                total_score: 84,
                maximum_score: 100,
                factors: vec![ReadinessFactorScore {
                    factor: "hardware".into(),
                    score: 84,
                    weight: 20,
                    weighted: 16.8,
                }],
            },
        ],
    };

    let path = PathBuf::from(std::env::temp_dir()).join(format!(
        "spanda-readiness-trend-{}.json",
        std::process::id()
    ));
    save_readiness_history(&path, &history).unwrap();

    history = load_readiness_history(&path);
    let report = analyze_readiness_trends(&history, "rover.sd", Some(7), 80);
    let trend = report
        .overall_trend
        .expect("overall trend should be present");
    assert!(trend.slope_per_day > 0.0);
    let forecast = report.forecast.expect("forecast should be present");
    assert!(forecast.predicted_score >= 84);
    assert!(!forecast.risk_warning);

    let _ = std::fs::remove_file(&path);
}

#[test]
fn parse_forecast_horizon_accepts_day_suffix() {
    assert_eq!(parse_forecast_horizon("7d"), Some(7));
    assert_eq!(parse_forecast_horizon("14"), Some(14));
    assert_eq!(parse_forecast_horizon("0d"), None);
}
