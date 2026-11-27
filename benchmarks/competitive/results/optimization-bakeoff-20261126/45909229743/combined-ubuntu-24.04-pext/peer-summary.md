| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 5.15x | 1.73x | 1.78x | 2.75x | 2.06x | 30/30 |
| materialize | 1.01x | 2.01x | 1.66x | 1.30x | 2.33x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.28x | 1.63x | 1.53x | 2.56x | 1.98x | 30/30 |
| perft_full_d3 | 1.24x | 1.69x | 1.87x | 1.65x | 2.43x | 2.26x | 30/30 |
| fen_parse | 0.87x | 1.30x | 1.34x | 1.07x | 2.06x | 1.20x | 30/30 |
| fen_write_native_ep | 1.00x | 4.10x | 9.87x | 1.89x | 29.92x | 2.22x | 30/30 |
| snapshot | 1.00x | 1.97x | 1.25x | 5.78x | 1.25x | 1.30x | 30/30 |
| san_parse_all | 1.02x | 2.14x | N/A | 4.63x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.70x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.32x | 1.33x | 4.63x | 1.47x | 1.64x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 82.53x | 144.35x | 235.54x | 152.89x | 5/5 |
| chess960_materialize | 0.98x | 1.58x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.88x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.99x | 1.91x | 1.52x | 2.88x | 2.48x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.56x | 1.18x | 2.05x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.40x | 3.17x | 2.49x | 3.27x | 1.81x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.57x | 1.95x | 2.57x | 2.82x | 9/10 |
| replay_checked_native | 0.98x | 2.55x | 1.09x | 8.37x | 8.43x | 9.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
