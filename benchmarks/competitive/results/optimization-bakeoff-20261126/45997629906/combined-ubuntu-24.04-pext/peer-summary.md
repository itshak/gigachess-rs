| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.61x | 1.87x | 1.64x | 2.51x | 1.92x | 30/30 |
| materialize | 0.94x | 2.15x | 1.56x | 1.76x | 2.78x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.94x | 1.53x | 1.19x | 2.37x | 1.88x | 29/30 |
| perft_full_d3 | 1.01x | 1.38x | 1.55x | 1.55x | 2.01x | 1.67x | 30/30 |
| fen_parse | 1.01x | 1.34x | 1.40x | 1.07x | 1.57x | 1.14x | 29/30 |
| fen_write_native_ep | 1.00x | 4.49x | 10.39x | 2.15x | 30.44x | 2.68x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 1.91x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.17x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.12x | 1.34x | 4.06x | 1.34x | 1.68x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.12x | 88.43x | 153.23x | 205.77x | 159.96x | 5/5 |
| chess960_materialize | 1.02x | 1.77x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.53x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 33.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.76x | 18.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 5.09x | 1.88x | 1.05x | 2.32x | 2.11x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.28x | 1.08x | 1.75x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.25x | 3.03x | 2.24x | 2.95x | 1.56x | 10/10 |
| replay_trusted_native | 0.92x | 0.84x | 1.44x | 1.47x | 2.09x | 2.38x | 9/10 |
| replay_checked_native | 0.95x | 2.06x | 1.06x | 8.66x | 8.43x | 9.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
