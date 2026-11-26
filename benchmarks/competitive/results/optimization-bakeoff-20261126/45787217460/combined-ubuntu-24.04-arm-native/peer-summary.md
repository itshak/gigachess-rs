| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.83x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 0.99x | 1.81x | 1.44x | 1.24x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.53x | 1.37x | 1.95x | 1.99x | 30/30 |
| perft_full_d3 | 0.93x | 1.07x | 1.16x | 0.97x | 1.40x | 1.37x | 24/30 |
| fen_parse | 1.01x | 1.26x | 1.34x | 1.31x | 1.80x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.36x | 1.57x | 30.08x | 2.38x | 30/30 |
| snapshot | 0.94x | 1.10x | 0.99x | 1.44x | 0.98x | 0.89x | 13/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.60x | N/A | 2.49x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.06x | 1.25x | 3.41x | 1.03x | 1.02x | 26/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.19x | 70.97x | 121.47x | 160.19x | 128.82x | 5/5 |
| chess960_materialize | 1.01x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.60x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.75x | 1.82x | 1.48x | 2.15x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.50x | 1.21x | 1.89x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.92x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 0.51x | 1.33x | 1.63x | 1.77x | 1.86x | 8/10 |
| replay_checked_native | 1.02x | 2.06x | 1.01x | 6.70x | 6.05x | 7.79x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
