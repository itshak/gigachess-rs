| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.29x | 1.45x | 1.49x | 2.29x | 1.72x | 30/30 |
| materialize | 1.01x | 1.89x | 1.57x | 1.24x | 2.18x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.64x | 1.40x | 1.25x | 2.28x | 1.71x | 30/30 |
| perft_full_d3 | 1.02x | 1.28x | 1.43x | 1.27x | 2.10x | 1.68x | 30/30 |
| fen_parse | 1.14x | 1.41x | 1.51x | 1.19x | 2.22x | 1.40x | 30/30 |
| fen_write_native_ep | 1.01x | 4.15x | 10.10x | 1.74x | 28.06x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.98x | 1.25x | 3.51x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.02x | 2.16x | N/A | 4.54x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.25x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.28x | 1.32x | 4.57x | 1.45x | 1.60x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.20x | 141.49x | 221.47x | 156.05x | 5/5 |
| chess960_materialize | 0.99x | 1.52x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.63x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.12x | 1.61x | 1.30x | 2.48x | 2.09x | 10/10 |
| corpus_materialize | 1.01x | 1.55x | 1.48x | 1.14x | 2.00x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.49x | 3.27x | 2.44x | 3.25x | 1.72x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.57x | 1.87x | 2.48x | 2.78x | 9/10 |
| replay_checked_native | 1.00x | 2.33x | 1.02x | 7.93x | 7.85x | 9.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
