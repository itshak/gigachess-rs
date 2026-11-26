| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.35x | 1.55x | 1.47x | 2.15x | 1.86x | 30/30 |
| materialize | 1.01x | 1.95x | 1.33x | 1.21x | 2.23x | 1.50x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.64x | 1.51x | 1.25x | 2.05x | 1.83x | 30/30 |
| perft_full_d3 | 0.98x | 1.46x | 1.80x | 1.25x | 1.83x | 1.78x | 30/30 |
| fen_parse | 1.07x | 1.36x | 1.57x | 1.39x | 2.37x | 1.63x | 30/30 |
| fen_write_native_ep | 1.00x | 4.14x | 8.04x | 1.61x | 31.68x | 1.95x | 30/30 |
| snapshot | 1.00x | 2.03x | 1.05x | 4.07x | 1.00x | 1.02x | 24/30 |
| san_parse_all | 1.04x | 2.14x | N/A | 5.63x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.85x | N/A | 3.85x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.09x | 1.27x | 8.58x | 1.34x | 1.50x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.41x | 149.23x | 230.42x | 178.14x | 5/5 |
| chess960_materialize | 1.01x | 1.54x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.68x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.87x | 1.68x | 1.25x | 2.26x | 2.17x | 10/10 |
| corpus_materialize | 0.99x | 1.45x | 1.18x | 1.07x | 1.85x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.74x | 2.16x | 1.88x | 2.11x | 1.39x | 10/10 |
| replay_trusted_native | 1.01x | 0.74x | 1.42x | 1.81x | 2.30x | 2.66x | 8/10 |
| replay_checked_native | 1.00x | 2.21x | 1.01x | 7.49x | 7.45x | 8.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
