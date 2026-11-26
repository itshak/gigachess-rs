| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.13x | 1.78x | 1.50x | 2.37x | 1.82x | 30/30 |
| materialize | 0.96x | 2.01x | 1.64x | 1.64x | 2.66x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.66x | 1.49x | 1.14x | 2.28x | 1.80x | 28/30 |
| perft_full_d3 | 1.25x | 1.66x | 2.00x | 1.85x | 2.51x | 2.07x | 30/30 |
| fen_parse | 1.00x | 1.51x | 1.38x | 1.17x | 1.48x | 1.26x | 30/30 |
| fen_write_native_ep | 0.95x | 4.71x | 9.67x | 1.97x | 26.88x | 2.38x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.48x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.92x | N/A | 2.06x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.35x | 4.03x | 1.44x | 1.65x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.80x | 0.99x | 78.70x | 144.24x | 172.95x | 156.05x | 4/5 |
| chess960_materialize | 0.91x | 1.59x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.43x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.14x | 23.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.79x | 4.18x | 1.58x | 0.87x | 1.93x | 1.75x | 8/10 |
| corpus_materialize | 0.99x | 1.34x | 1.25x | 1.04x | 1.68x | 1.15x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.24x | 2.85x | 2.13x | 2.78x | 1.49x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.57x | 1.61x | 2.13x | 2.82x | 9/10 |
| replay_checked_native | 1.02x | 2.20x | 1.16x | 9.28x | 9.02x | 10.82x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
