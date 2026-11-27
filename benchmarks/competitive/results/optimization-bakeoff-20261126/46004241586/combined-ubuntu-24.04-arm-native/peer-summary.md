| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.44x | 1.69x | 1.82x | 2.20x | 2.19x | 30/30 |
| materialize | 1.02x | 1.98x | 1.54x | 1.38x | 2.18x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.01x | 1.64x | 1.49x | 2.09x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.82x | 1.51x | 2.22x | 2.10x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.34x | 1.29x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.45x | 9.22x | 1.61x | 29.73x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.11x | 1.04x | 1.47x | 1.05x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.88x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.37x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 4.04x | 1.20x | 1.19x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.67x | 130.40x | 170.54x | 137.12x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.03x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.84x | 1.91x | 1.49x | 2.16x | 2.40x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.54x | 1.24x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.63x | 1.68x | 1.97x | 2.23x | 2.25x | 8/10 |
| replay_checked_native | 1.02x | 2.36x | 1.15x | 7.88x | 6.88x | 8.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
