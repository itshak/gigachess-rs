| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.47x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.82x | 1.45x | 1.25x | 2.03x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.52x | 1.39x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.07x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.26x | 1.36x | 1.30x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.61x | 9.36x | 1.59x | 29.65x | 2.39x | 30/30 |
| snapshot | 1.02x | 1.25x | 0.94x | 1.59x | 0.91x | 1.02x | 18/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.82x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.13x | 1.30x | 3.66x | 1.08x | 1.10x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.18x | 76.30x | 133.43x | 173.81x | 140.04x | 5/5 |
| chess960_materialize | 0.99x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.78x | 1.83x | 1.49x | 2.15x | 2.37x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.49x | 1.20x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.09x | 2.89x | 2.07x | 2.75x | 1.44x | 10/10 |
| replay_trusted_native | 0.98x | 0.50x | 1.30x | 1.58x | 1.75x | 1.79x | 8/10 |
| replay_checked_native | 0.93x | 1.88x | 0.92x | 6.15x | 5.47x | 7.07x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
