| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.52x | 1.43x | 2.10x | 1.82x | 30/30 |
| materialize | 1.10x | 2.04x | 1.63x | 1.15x | 2.26x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.78x | 1.45x | 1.16x | 2.18x | 2.06x | 30/30 |
| perft_full_d3 | 1.10x | 1.17x | 1.36x | 1.02x | 1.76x | 1.73x | 28/30 |
| fen_parse | 1.15x | 1.27x | 1.45x | 1.10x | 1.92x | 1.15x | 29/30 |
| fen_write_native_ep | 1.01x | 3.86x | 8.53x | 1.47x | 25.36x | 1.93x | 30/30 |
| snapshot | 1.16x | 2.48x | 1.49x | 4.48x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.77x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.13x | N/A | 2.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.22x | 1.76x | 1.50x | 3.72x | 1.52x | 2.03x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.25x | 83.95x | 138.66x | 213.41x | 135.33x | 5/5 |
| chess960_materialize | 1.04x | 1.59x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.65x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 35.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 22.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.88x | 5.44x | 1.63x | 1.08x | 2.14x | 2.02x | 9/10 |
| corpus_materialize | 0.89x | 1.57x | 1.49x | 1.04x | 2.05x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.28x | 2.89x | 2.35x | 2.91x | 1.64x | 10/10 |
| replay_trusted_native | 0.97x | 0.64x | 1.22x | 1.39x | 2.16x | 2.60x | 8/10 |
| replay_checked_native | 1.01x | 2.01x | 0.92x | 6.99x | 7.04x | 8.49x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
