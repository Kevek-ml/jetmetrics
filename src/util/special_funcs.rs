/// Regularized incomplete beta function I_x(a, b).
///
/// Used for t-test p-values. Must match scipy.special.betainc to 8 decimal places.
pub fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    let _ = (x, a, b);
    todo!("Phase 2: delegate to statrs::distribution::Beta or implement via continued fraction")
}

/// Survival function of the KS distribution for two-sample test.
///
/// P(D_n,m > d) where n, m are sample sizes and d is the observed statistic.
/// Must match scipy.stats.ks_2samp p-value to 8 decimal places.
pub fn ks_two_sample_sf(d: f64, n: usize, m: usize) -> f64 {
    let _ = (d, n, m);
    todo!("Phase 2: implement via Kolmogorov distribution CDF")
}

/// Standard normal CDF Φ(x).
///
/// Used for Mann-Whitney U normal approximation.
pub fn normal_sf(z: f64) -> f64 {
    let _ = z;
    todo!("Phase 2: delegate to statrs::distribution::Normal")
}
