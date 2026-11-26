| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.67x | 1.80x | 2.17x | 2.15x | 30/30 |
| materialize | 0.99x | 1.93x | 1.54x | 1.34x | 2.14x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.58x | 1.42x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.60x | 1.33x | 1.96x | 1.84x | 30/30 |
| fen_parse | 0.99x | 1.23x | 1.33x | 1.27x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.33x | 1.58x | 29.93x | 2.42x | 30/30 |
| snapshot | 1.01x | 1.16x | 1.01x | 1.49x | 1.01x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.45x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.04x | 1.22x | 1.21x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.67x | 130.90x | 170.33x | 137.68x | 5/5 |
| chess960_materialize | 0.99x | 1.67x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.42x | 1.71x | 1.36x | 2.03x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.23x | 1.92x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.64x | 1.66x | 2.01x | 2.25x | 2.31x | 8/10 |
| replay_checked_native | 1.00x | 2.33x | 1.13x | 7.62x | 6.80x | 8.75x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
