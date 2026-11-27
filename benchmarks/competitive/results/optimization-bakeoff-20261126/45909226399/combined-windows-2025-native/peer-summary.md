| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.75x | 1.70x | 1.62x | 2.35x | 2.04x | 30/30 |
| materialize | 0.98x | 2.00x | 1.38x | 1.25x | 2.33x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.94x | 1.58x | 1.36x | 2.24x | 2.00x | 30/30 |
| perft_full_d3 | 1.01x | 1.42x | 1.51x | 1.36x | 1.98x | 1.93x | 30/30 |
| fen_parse | 0.91x | 1.15x | 1.23x | 1.19x | 1.96x | 1.36x | 30/30 |
| fen_write_native_ep | 1.00x | 4.09x | 8.18x | 1.60x | 32.41x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.81x | 1.00x | 4.09x | 1.01x | 1.03x | 23/30 |
| san_parse_all | 1.02x | 2.09x | N/A | 5.55x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.19x | N/A | 3.92x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.31x | 1.40x | 9.46x | 1.47x | 1.67x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.45x | 98.95x | 151.96x | 234.30x | 198.81x | 5/5 |
| chess960_materialize | 0.89x | 1.45x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 2.68x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 23.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 19.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.23x | 1.95x | 1.35x | 2.45x | 2.35x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.26x | 1.15x | 2.01x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.78x | 2.24x | 1.83x | 2.19x | 1.43x | 10/10 |
| replay_trusted_native | 0.99x | 0.84x | 1.58x | 2.05x | 2.56x | 2.97x | 9/10 |
| replay_checked_native | 1.00x | 2.41x | 1.03x | 8.02x | 7.88x | 9.54x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
