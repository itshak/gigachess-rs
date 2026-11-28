| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.56x | 1.94x | 1.64x | 2.51x | 1.99x | 30/30 |
| materialize | 1.00x | 2.21x | 1.68x | 1.73x | 2.62x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.93x | 1.61x | 1.25x | 2.43x | 1.94x | 29/30 |
| perft_full_d3 | 1.17x | 1.58x | 1.88x | 1.76x | 2.31x | 1.96x | 30/30 |
| fen_parse | 0.99x | 1.31x | 1.37x | 1.16x | 1.47x | 1.14x | 30/30 |
| fen_write_native_ep | 1.00x | 4.94x | 10.07x | 1.89x | 28.21x | 2.17x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.30x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.92x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.03x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.15x | 1.35x | 4.09x | 1.35x | 1.70x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.25x | 162.76x | 193.68x | 158.55x | 5/5 |
| chess960_materialize | 0.99x | 1.73x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.50x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.63x | 1.62x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.21x | 6.34x | 2.37x | 1.31x | 3.00x | 2.65x | 10/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.05x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.29x | 2.94x | 2.24x | 2.91x | 1.55x | 10/10 |
| replay_trusted_native | 0.89x | 0.76x | 1.43x | 1.40x | 1.91x | 2.25x | 8/10 |
| replay_checked_native | 0.92x | 2.02x | 1.06x | 8.42x | 8.39x | 9.69x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
