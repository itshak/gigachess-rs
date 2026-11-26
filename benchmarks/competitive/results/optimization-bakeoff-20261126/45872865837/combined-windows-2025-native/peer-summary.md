| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.84x | 1.71x | 1.65x | 2.41x | 2.08x | 30/30 |
| materialize | 1.02x | 1.99x | 1.37x | 1.26x | 2.31x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.05x | 1.62x | 1.39x | 2.32x | 2.06x | 30/30 |
| perft_full_d3 | 1.22x | 1.70x | 1.79x | 1.60x | 2.34x | 2.28x | 30/30 |
| fen_parse | 1.10x | 1.32x | 1.52x | 1.36x | 2.42x | 1.52x | 30/30 |
| fen_write_native_ep | 1.01x | 4.23x | 8.85x | 1.61x | 32.62x | 1.99x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.05x | 4.26x | 1.09x | 1.02x | 25/30 |
| san_parse_all | 1.00x | 2.05x | N/A | 5.58x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.12x | N/A | 3.96x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 2.30x | 1.41x | 9.46x | 1.47x | 1.65x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.25x | 78.65x | 150.58x | 247.41x | 172.20x | 5/5 |
| chess960_materialize | 1.02x | 1.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.03x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 28.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.09x | 21.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.39x | 1.81x | 1.36x | 2.47x | 2.37x | 10/10 |
| corpus_materialize | 0.98x | 1.52x | 1.24x | 1.15x | 1.94x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.77x | 2.27x | 1.80x | 2.20x | 1.44x | 10/10 |
| replay_trusted_native | 0.94x | 0.81x | 1.53x | 1.97x | 2.43x | 3.16x | 9/10 |
| replay_checked_native | 1.01x | 2.44x | 1.15x | 8.91x | 8.72x | 9.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
