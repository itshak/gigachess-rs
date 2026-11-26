| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.66x | 1.67x | 1.60x | 2.50x | 2.01x | 30/30 |
| materialize | 1.00x | 1.95x | 1.68x | 1.25x | 2.26x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.07x | 1.60x | 1.41x | 2.42x | 2.01x | 30/30 |
| perft_full_d3 | 1.00x | 1.38x | 1.53x | 1.33x | 1.96x | 1.92x | 30/30 |
| fen_parse | 1.07x | 1.36x | 1.47x | 1.17x | 2.18x | 1.28x | 30/30 |
| fen_write_native_ep | 1.00x | 4.15x | 10.00x | 1.80x | 28.38x | 2.24x | 30/30 |
| snapshot | 0.78x | 1.55x | 0.97x | 2.57x | 1.18x | 0.97x | 18/30 |
| san_parse_all | 0.99x | 2.08x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.70x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.34x | 1.34x | 4.66x | 1.49x | 1.63x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 84.32x | 145.51x | 241.11x | 155.86x | 5/5 |
| chess960_materialize | 1.00x | 1.58x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.80x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 29.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 20.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.44x | 1.79x | 1.36x | 2.76x | 2.38x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.51x | 1.14x | 1.99x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.41x | 3.22x | 2.45x | 3.11x | 1.73x | 10/10 |
| replay_trusted_native | 0.97x | 0.88x | 1.62x | 1.93x | 2.57x | 2.89x | 9/10 |
| replay_checked_native | 0.97x | 2.42x | 1.09x | 8.15x | 8.23x | 9.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
