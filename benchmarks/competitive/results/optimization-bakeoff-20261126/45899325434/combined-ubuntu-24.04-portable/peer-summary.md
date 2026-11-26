| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.86x | 1.63x | 1.42x | 1.87x | 1.37x | 30/30 |
| materialize | 0.99x | 2.37x | 1.84x | 1.87x | 2.75x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.04x | 1.61x | 1.20x | 1.99x | 1.80x | 29/30 |
| perft_full_d3 | 1.18x | 1.61x | 1.90x | 1.67x | 2.19x | 1.88x | 30/30 |
| fen_parse | 1.01x | 1.35x | 1.29x | 1.08x | 1.34x | 1.09x | 30/30 |
| fen_write_native_ep | 1.02x | 3.04x | 9.85x | 1.85x | 28.11x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.49x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.24x | 1.43x | 4.20x | 1.38x | 1.58x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 61.64x | 107.95x | 119.44x | 96.82x | 5/5 |
| chess960_materialize | 1.07x | 1.84x | 1.80x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.07x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 43.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.04x | 21.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.62x | 2.28x | 1.26x | 2.39x | 3.16x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.42x | 1.14x | 1.84x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.95x | 2.19x | 2.89x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.71x | 1.61x | 2.20x | 2.05x | 9/10 |
| replay_checked_native | 1.02x | 2.08x | 1.22x | 9.06x | 8.81x | 10.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
