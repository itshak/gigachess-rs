| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.70x | 1.58x | 1.62x | 2.42x | 1.87x | 30/30 |
| materialize | 0.97x | 1.95x | 1.59x | 1.24x | 2.23x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.87x | 3.50x | 1.34x | 1.18x | 2.07x | 1.64x | 30/30 |
| perft_full_d3 | 0.87x | 1.21x | 1.32x | 1.18x | 1.72x | 1.57x | 30/30 |
| fen_parse | 0.83x | 1.15x | 1.30x | 1.03x | 1.92x | 1.10x | 28/30 |
| fen_write_native_ep | 0.92x | 3.70x | 9.17x | 1.60x | 26.39x | 1.92x | 30/30 |
| snapshot | 1.00x | 2.19x | 1.25x | 3.50x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.88x | 1.92x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.44x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 2.15x | 1.24x | 4.09x | 1.41x | 1.57x | 30/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 1.50x | 1.88x | 120.09x | 216.65x | 352.05x | 218.71x | 5/5 |
| chess960_materialize | 0.94x | 1.58x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.86x | 2.49x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 31.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.09x | 23.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.49x | 1.72x | 1.36x | 2.61x | 2.25x | 10/10 |
| corpus_materialize | 0.98x | 1.65x | 1.57x | 1.19x | 2.05x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.53x | 3.33x | 2.55x | 3.24x | 1.82x | 10/10 |
| replay_trusted_native | 0.91x | 1.06x | 1.45x | 1.79x | 2.72x | 3.06x | 9/10 |
| replay_checked_native | 0.84x | 2.15x | 0.94x | 7.16x | 7.25x | 8.72x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
