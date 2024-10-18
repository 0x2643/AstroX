# AstroX

[![GitHub license](https://img.shields.io/github/license/spectre-project/rusty-spectrex.svg)](https://github.com/0x2643/AstroX/blob/Main/LICENSE)

## Overview

AstroX is a fork of [SpectreX](https://github.com/spectre-project/rusty-spectrex.git)
algorithm, a proof-of-work (PoW) system based on the Burrows-Wheeler
transform (BWT). AstroX removes the BWT part to simplify integration for both pools and mining application programmers. It also fixes several parts of the algorithm that reduced randomness.

## Benchmarks

Included is a simple computation benchmark using [Criterion](https://github.com/bheisler/criterion.rs).
This benchmark helps verify any performance improvements or
degradations if any calculation steps have been modified. You can run
it using `cargo bench`, and it will return the following results:

```
AstroX                  time:   [196.53 µs 198.31 µs 200.40 µs]
                        change: [+1.8320% +2.5729% +3.4270%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
```
