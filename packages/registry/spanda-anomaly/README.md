# spanda-anomaly

Official Spanda package for mission assurance: **anomaly**.

Core interfaces live in `spanda-assurance`; this package provides optional learned-detector scaffolds and provider hooks.

## Usage

Declare an ML-backed detector in your program:

```spanda
import assurance.anomaly;

anomaly_detector NavigationML {
    learned backend assurance.anomaly;
    expected localization.confidence >= 0.80;
}
```

Run `spanda anomaly scan program.sd` — learned models appear in the report when a backend is declared or imported.

At runtime, `scan_learned(detector, observed, volatility)` uses threshold + EMA volatility. Set **`SPANDA_ANOMALY_ONNX_MODEL_PATH`** (or `SPANDA_ONNX_MODEL_PATH`) for ONNX inference via the Python bridge (`onnxruntime` optional; 2-feature input `[observed, volatility]`).

See `examples/anomaly/learned_navigation.sd` and `examples/showcase/assurance/rover.sd`.
