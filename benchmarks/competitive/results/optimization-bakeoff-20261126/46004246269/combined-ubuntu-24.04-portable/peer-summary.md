| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.06x | 1.74x | 1.51x | 2.00x | 1.45x | 30/30 |
| materialize | 1.00x | 2.31x | 1.87x | 1.94x | 2.82x | 2.02x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.24x | 1.70x | 1.27x | 2.11x | 1.90x | 30/30 |
| perft_full_d3 | 1.17x | 1.59x | 1.84x | 1.66x | 2.17x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.46x | 1.46x | 1.19x | 1.46x | 1.15x | 30/30 |
| fen_write_native_ep | 1.01x | 3.01x | 10.09x | 2.03x | 27.55x | 2.55x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 25/30 |
| san_parse_all | 0.93x | 1.80x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.40x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.83x | 1.43x | 4.29x | 1.38x | 1.58x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.16x | 112.99x | 121.77x | 96.40x | 5/5 |
| chess960_materialize | 1.01x | 1.70x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.13x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 39.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 20.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 4.90x | 2.44x | 1.35x | 2.52x | 3.34x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.42x | 1.14x | 1.84x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.93x | 2.97x | 2.18x | 2.92x | 1.52x | 10/10 |
| replay_trusted_native | 1.01x | 0.88x | 1.67x | 1.66x | 2.24x | 2.06x | 9/10 |
| replay_checked_native | 1.01x | 2.06x | 1.14x | 8.89x | 8.94x | 10.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
