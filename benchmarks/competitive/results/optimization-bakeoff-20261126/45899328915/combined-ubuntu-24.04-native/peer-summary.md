| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.34x | 1.58x | 1.52x | 2.38x | 1.84x | 30/30 |
| materialize | 0.95x | 2.07x | 1.67x | 1.13x | 2.13x | 1.45x | 29/30 |
| perft_bulk_d4 | 0.99x | 4.08x | 1.60x | 1.31x | 2.55x | 2.16x | 30/30 |
| perft_full_d3 | 0.92x | 1.40x | 1.44x | 1.09x | 2.03x | 2.17x | 29/30 |
| fen_parse | 0.92x | 1.37x | 1.49x | 1.22x | 2.14x | 1.24x | 30/30 |
| fen_write_native_ep | 0.89x | 3.65x | 8.87x | 1.64x | 24.84x | 2.09x | 30/30 |
| snapshot | 0.67x | 1.57x | 1.19x | 2.49x | 1.18x | 1.00x | 27/30 |
| san_parse_all | 0.95x | 2.11x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 9.87x | N/A | 2.00x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.96x | 1.85x | 4.89x | 1.81x | 1.71x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 0.83x | 1.17x | 83.47x | 135.61x | 202.00x | 130.32x | 5/5 |
| chess960_materialize | 1.01x | 1.88x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.94x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 21.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 18.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.35x | 1.75x | 1.30x | 2.33x | 2.04x | 10/10 |
| corpus_materialize | 0.98x | 1.87x | 1.72x | 1.23x | 2.84x | 1.57x | 10/10 |
| corpus_uci_sorted | 0.96x | 2.28x | 3.08x | 2.30x | 2.95x | 1.65x | 10/10 |
| replay_trusted_native | 0.87x | 0.94x | 1.66x | 1.86x | 3.31x | 3.97x | 9/10 |
| replay_checked_native | 0.90x | 2.58x | 1.01x | 5.68x | 6.58x | 8.26x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
