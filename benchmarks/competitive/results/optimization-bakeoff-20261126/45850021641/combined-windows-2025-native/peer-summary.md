| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.90x | 1.75x | 1.67x | 2.42x | 2.11x | 30/30 |
| materialize | 0.96x | 1.98x | 1.34x | 1.26x | 2.25x | 1.52x | 30/30 |
| perft_bulk_d4 | 0.97x | 4.11x | 1.67x | 1.37x | 2.26x | 2.01x | 30/30 |
| perft_full_d3 | 0.97x | 1.46x | 1.51x | 1.36x | 1.97x | 1.94x | 30/30 |
| fen_parse | 1.02x | 1.46x | 1.44x | 1.41x | 2.25x | 1.47x | 30/30 |
| fen_write_native_ep | 1.02x | 4.13x | 8.37x | 1.58x | 31.18x | 2.02x | 30/30 |
| snapshot | 1.03x | 1.82x | 1.12x | 4.36x | 1.10x | 1.06x | 28/30 |
| san_parse_all | 1.00x | 2.12x | N/A | 5.59x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.58x | N/A | 3.76x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.30x | 1.40x | 10.04x | 1.47x | 1.66x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 82.49x | 171.47x | 254.87x | 167.25x | 5/5 |
| chess960_materialize | 0.99x | 1.67x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 2.75x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 30.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 23.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.21x | 1.79x | 1.39x | 2.59x | 2.44x | 10/10 |
| corpus_materialize | 0.95x | 1.70x | 1.23x | 1.14x | 1.95x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.45x | 1.88x | 2.30x | 1.49x | 10/10 |
| replay_trusted_native | 1.02x | 0.86x | 1.57x | 2.29x | 2.55x | 3.02x | 9/10 |
| replay_checked_native | 0.98x | 2.28x | 1.07x | 8.39x | 8.13x | 10.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
