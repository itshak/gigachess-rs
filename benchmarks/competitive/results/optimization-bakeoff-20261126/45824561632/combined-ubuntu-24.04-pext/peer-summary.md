| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.97x | 1.67x | 1.46x | 2.27x | 1.70x | 30/30 |
| materialize | 1.07x | 2.11x | 1.58x | 1.66x | 2.61x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.31x | 1.31x | 1.03x | 2.05x | 1.60x | 28/30 |
| perft_full_d3 | 0.88x | 1.32x | 1.48x | 1.45x | 1.89x | 1.55x | 30/30 |
| fen_parse | 1.02x | 1.29x | 1.36x | 1.19x | 1.48x | 1.15x | 30/30 |
| fen_write_native_ep | 0.99x | 4.60x | 10.00x | 1.88x | 29.54x | 2.17x | 30/30 |
| snapshot | 0.61x | 1.50x | 0.93x | 2.65x | 0.94x | 1.00x | 17/30 |
| san_parse_all | 0.95x | 1.84x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.06x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.10x | 1.34x | 3.92x | 1.30x | 1.64x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.92x | 1.30x | 81.35x | 148.99x | 172.60x | 141.95x | 5/5 |
| chess960_materialize | 0.88x | 1.51x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.26x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 39.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.22x | 21.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.64x | 1.73x | 0.98x | 2.14x | 1.94x | 8/10 |
| corpus_materialize | 0.99x | 1.35x | 1.25x | 1.04x | 1.68x | 1.13x | 9/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 3.01x | 2.24x | 2.92x | 1.54x | 10/10 |
| replay_trusted_native | 0.95x | 0.83x | 1.52x | 1.52x | 2.10x | 2.42x | 9/10 |
| replay_checked_native | 0.91x | 1.84x | 0.96x | 7.68x | 7.46x | 8.71x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
