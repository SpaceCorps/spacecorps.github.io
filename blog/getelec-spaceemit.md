---
title: "47,000× faster electron emission: spaceemit vs GETELEC"
description: "Custom Rust physics software built for a client: 17 µs per emission point instead of 650 ms, agreeing with GETELEC to a median 0.0007 % in current density."
author: "SpaceCorps"
date: "2026-10-16"
canonical: "https://spacecorps.github.io/blog/getelec-spaceemit.html"
---

# 47,000× faster electron emission: spaceemit vs GETELEC

A client needed thermal-field electron emission for metal nanotips across millions of field, geometry and temperature combinations. The reference tool took ~650 ms per point. SpaceCorps built `spaceemit` (Rust, numerically exact barrier transmission, clean-room): 17 µs per point.

Benchmark: spaceemit 0.2.0 vs GETELEC (2026 Python/numba edition, settings shipped in its examples, untuned), Apple M4 Max, 16 threads, 2026-10-05, 2,456 metal points.

| Workload | GETELEC | spaceemit | Speed-up |
|---|---:|---:|---:|
| One point, 1 thread (median of 40) | 650 ms | 17 µs | ~38,000× |
| One point, GETELEC default threading | 372 ms | 17 µs | ~22,000× |
| 2,456 points, all cores | 224 s | 4.7 ms | ~47,000× |
| 10^6 points, all cores (extrapolated) | ~25 h | 1.9 s | ~47,000× |

Agreement over 2,199 points with J ≥ 1e-20 A/nm²: median |ΔJ| 0.0007 % (p90 0.003 %, max 0.4 %); median |ΔP_N| 0.003 %; sign of Nottingham heat matches everywhere.

Caveats: GETELEC solves a more general problem (semiconductor emitters were not compared); coarser GETELEC grids would be faster and less accurate; timings are in-process after JIT warm-up.

Licence: PolyForm Shield 1.0.0. Registry: https://spacecorps-registry.sliplane.app. Full write-up: https://spacecorps.github.io/blog/getelec-spaceemit.html
