# jetmetrics

**Arrow-native, Rust-based statistical metrics for ML monitoring.**

Compute drift, performance, and statistical metrics with zero-copy interchange using Apache Arrow.

## Status

**Design Phase** — Architecture documented. MVP development to follow.

## Vision

**jetmetrics** is the first infrastructure-grade metrics library that doesn't force a conversion boundary:
- **Arrow-native interface** — accepts PyArrow arrays directly, zero-copy from any Arrow-speaking framework
- **Rust core** — memory-safe, no GC, single `.so` binary, accurate p-values
- **Streaming accumulators** — designed from day one for out-of-core computation
- **Accuracy-first** — all p-values validated to 8 decimal places, bitwise-reproducible results

## MVP (v0.1) Metrics

| Category | Metrics |
|---|---|
| **Drift — Distances** | PSI, Wasserstein, Hellinger, JS divergence, TVD |
| **Drift — Tests** | KS test, t-test (independent), Mann-Whitney U |
| **Performance — Classification** | accuracy, f1, precision, recall, AUC, AUCPR, log_loss |
| **Performance — Regression** | MAE, MSE, MAPE, R², RMSE |
| **Dataset-level** | row_count, column_count, null_count (per column) |

## Architecture

- **Language**: Rust (memory-safe, no GC overhead)
- **Core**: arrow-rs (Apache Arrow Rust implementation)
- **Bindings**: PyO3 + maturin for Python FFI
- **Streaming**: Accumulator trait from day one for out-of-core computation (v0.2+)


## Performance

The performance advantage of jetmetrics is conditional on your data format:

**Arrow-native users** (Polars, pandas 2.0 Arrow backend): jetmetrics accepts your data zero-copy. Reference implementations require a full array copy and type conversion before computation — at n=10k that conversion cost often exceeds the computation itself. Expected end-to-end speedup: **3-10× depending on metric**.

**Existing numpy users**: there is no raw computation speedup. jetmetrics is not a faster algorithm for numpy arrays — it is a boundary-free alternative for users who are already in the Arrow ecosystem.

| Metric | Source of speedup |
|---|---|
| PSI, Hellinger, TVD, JS | Eliminated Arrow→numpy conversion (histogram loop itself is sequential) |
| Wasserstein | Eliminated conversion; sort cost is O(n log n) in both |
| KS test | Eliminated conversion + faster special function evaluation in Rust |
| AUC | Eliminated conversion; sort + trapezoid rule comparable |
| t-test | Eliminated conversion; Welford's algorithm comparable |

*All figures are estimates pending benchmarks. Actual speedup depends on array size, null density, and whether data is already in Arrow format.*

## Timeline

- **Phase 1 (6 weeks)**: Rust core, distance metrics, PyO3 binding
- **Phase 2 (4 weeks)**: Hypothesis tests, special functions, validation
- **Phase 3 (2 weeks)**: Performance + dataset-level metrics
- **Phase 4 (1 week)**: PyPI setup and release
- **v0.1 release**: ~13 weeks

## Development

```bash
git clone https://github.com/Kevek-ml/jetmetrics.git
cd jetmetrics

pip install -e ".[dev]"

# Build Rust extension (requires Rust toolchain)
maturin develop

pytest tests/
```

## Reference

- [Apache Arrow Spec](https://arrow.apache.org/docs/)
- [arrow-rs GitHub](https://github.com/apache/arrow-rs)
- [PyO3 Documentation](https://pyo3.rs/)
- [Maturin Documentation](https://www.maturin.rs/)

## License

Apache 2.0

---

**Author**: Kevek ML  
**Last Updated**: 2026-05-31
