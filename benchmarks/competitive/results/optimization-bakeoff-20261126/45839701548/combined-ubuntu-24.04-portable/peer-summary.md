| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.08x | 1.65x | 1.38x | 1.95x | 1.38x | 30/30 |
| materialize | 1.01x | 2.33x | 1.77x | 1.37x | 2.64x | 1.68x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.11x | 1.61x | 1.22x | 1.99x | 1.78x | 30/30 |
| perft_full_d3 | 1.07x | 1.21x | 1.45x | 1.15x | 1.63x | 1.51x | 29/30 |
| fen_parse | 1.06x | 1.60x | 1.56x | 1.19x | 1.93x | 1.25x | 30/30 |
| fen_write_native_ep | 1.00x | 2.70x | 8.47x | 1.47x | 25.11x | 1.98x | 30/30 |
| snapshot | 0.99x | 1.61x | 1.16x | 2.62x | 1.15x | 0.99x | 25/30 |
| san_parse_all | 1.02x | 1.78x | N/A | 3.71x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.54x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.06x | 1.35x | 5.00x | 1.25x | 1.25x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.29x | 58.77x | 80.36x | 118.66x | 82.74x | 5/5 |
| chess960_materialize | 1.01x | 1.84x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.30x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 31.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.77x | 0.80x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.18x | 24.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.23x | 1.79x | 1.07x | 1.89x | 2.76x | 9/10 |
| corpus_materialize | 1.00x | 1.72x | 1.56x | 1.17x | 2.32x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.05x | 2.82x | 2.30x | 2.88x | 1.61x | 10/10 |
| replay_trusted_native | 0.95x | 0.62x | 1.38x | 1.42x | 1.90x | 1.74x | 8/10 |
| replay_checked_native | 0.99x | 2.08x | 1.21x | 8.41x | 8.24x | 9.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
