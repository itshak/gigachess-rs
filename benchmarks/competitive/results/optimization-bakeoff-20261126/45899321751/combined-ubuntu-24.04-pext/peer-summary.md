| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.45x | 1.86x | 1.62x | 2.49x | 1.90x | 30/30 |
| materialize | 1.11x | 2.28x | 1.68x | 1.80x | 2.77x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.95x | 1.57x | 1.23x | 2.42x | 1.91x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.57x | 1.49x | 1.96x | 1.64x | 30/30 |
| fen_parse | 0.94x | 1.40x | 1.29x | 1.12x | 1.40x | 1.19x | 29/30 |
| fen_write_native_ep | 0.99x | 4.76x | 10.34x | 2.02x | 27.64x | 2.48x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.03x | 1.92x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.22x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.35x | 4.03x | 1.34x | 1.68x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.13x | 1.12x | 88.78x | 157.51x | 196.03x | 174.87x | 5/5 |
| chess960_materialize | 1.01x | 1.76x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.78x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 19.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.16x | 6.32x | 2.34x | 1.32x | 2.87x | 2.67x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.30x | 1.09x | 1.76x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 2.97x | 2.23x | 2.96x | 1.55x | 10/10 |
| replay_trusted_native | 1.02x | 0.91x | 1.68x | 1.64x | 2.30x | 2.63x | 9/10 |
| replay_checked_native | 0.99x | 2.18x | 1.19x | 9.13x | 8.84x | 10.25x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
