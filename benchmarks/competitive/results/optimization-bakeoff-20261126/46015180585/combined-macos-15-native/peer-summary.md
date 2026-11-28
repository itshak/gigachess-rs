| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.98x | 1.75x | 1.97x | 2.38x | 2.16x | 30/30 |
| materialize | 1.03x | 1.88x | 1.42x | 1.06x | 2.15x | 1.36x | 28/30 |
| perft_bulk_d4 | 1.08x | 4.29x | 1.60x | 1.34x | 1.98x | 2.02x | 30/30 |
| perft_full_d3 | 1.10x | 1.67x | 1.84x | 1.46x | 2.11x | 2.23x | 30/30 |
| fen_parse | 1.06x | 1.26x | 1.31x | 1.47x | 1.71x | 1.58x | 30/30 |
| fen_write_native_ep | 1.02x | 4.24x | 8.44x | 1.66x | 26.72x | 2.23x | 30/30 |
| snapshot | 1.17x | 1.83x | 1.24x | 2.71x | 1.18x | 1.01x | 24/30 |
| san_parse_all | 0.98x | 1.84x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 13.26x | N/A | 2.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.15x | 1.24x | 6.12x | 1.19x | 1.40x | 25/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.31x | N/A | N/A | 5/6 |
| startpos | 0.63x | 0.79x | 52.12x | 119.87x | 132.24x | 101.46x | 4/5 |
| chess960_materialize | 1.05x | 1.78x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 3.12x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.49x | 1.64x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.86x | 18.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 6.27x | 1.98x | 1.21x | 2.50x | 2.46x | 9/10 |
| corpus_materialize | 1.03x | 1.53x | 1.53x | 1.26x | 2.06x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.21x | 2.86x | 1.93x | 2.88x | 1.52x | 10/10 |
| replay_trusted_native | 0.94x | 0.65x | 1.73x | 1.71x | 1.83x | 2.02x | 8/10 |
| replay_checked_native | 0.85x | 2.57x | 1.22x | 10.46x | 8.50x | 10.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
