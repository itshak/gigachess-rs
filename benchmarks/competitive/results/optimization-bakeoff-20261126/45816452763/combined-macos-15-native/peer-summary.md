| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.82x | 3.79x | 1.27x | 1.47x | 1.83x | 1.52x | 29/30 |
| materialize | 0.88x | 1.91x | 1.49x | 1.14x | 2.28x | 1.55x | 28/30 |
| perft_bulk_d4 | 0.95x | 3.57x | 1.42x | 1.25x | 1.78x | 1.71x | 30/30 |
| perft_full_d3 | 0.93x | 1.52x | 1.77x | 1.35x | 1.92x | 1.95x | 30/30 |
| fen_parse | 0.87x | 1.12x | 1.34x | 1.25x | 1.77x | 1.40x | 28/30 |
| fen_write_native_ep | 0.93x | 4.33x | 8.85x | 1.76x | 29.35x | 2.22x | 30/30 |
| snapshot | 0.98x | 1.96x | 1.15x | 2.43x | 1.29x | 1.00x | 25/30 |
| san_parse_all | 0.89x | 1.66x | N/A | 3.63x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 12.12x | N/A | 2.72x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.04x | 1.18x | 5.27x | 1.03x | 1.26x | 23/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.16x | N/A | N/A | 4/6 |
| startpos | 0.91x | 1.06x | 71.92x | 137.93x | 192.31x | 159.49x | 5/5 |
| chess960_materialize | 0.93x | 1.48x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 2.73x | 1.09x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.19x | 39.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 19.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.97x | 1.80x | 1.12x | 2.20x | 2.30x | 9/10 |
| corpus_materialize | 1.02x | 1.30x | 1.26x | 0.97x | 1.80x | 1.06x | 8/10 |
| corpus_uci_sorted | 1.29x | 2.08x | 2.78x | 1.98x | 2.80x | 1.55x | 10/10 |
| replay_trusted_native | 1.05x | 0.53x | 1.52x | 1.60x | 1.57x | 1.81x | 8/10 |
| replay_checked_native | 0.98x | 2.28x | 1.04x | 8.25x | 7.10x | 9.06x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
