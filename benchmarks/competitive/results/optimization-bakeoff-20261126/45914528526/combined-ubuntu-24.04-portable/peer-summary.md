| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.20x | 1.64x | 1.47x | 2.07x | 1.43x | 30/30 |
| materialize | 1.00x | 2.42x | 1.97x | 1.34x | 2.57x | 1.72x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.12x | 1.63x | 1.29x | 2.05x | 1.76x | 30/30 |
| perft_full_d3 | 1.00x | 1.43x | 1.68x | 1.29x | 1.99x | 1.85x | 30/30 |
| fen_parse | 0.95x | 1.55x | 1.52x | 1.29x | 1.93x | 1.32x | 30/30 |
| fen_write_native_ep | 0.99x | 2.62x | 9.57x | 1.71x | 26.19x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.87x | 1.13x | 0.98x | 24/30 |
| san_parse_all | 0.98x | 2.21x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.45x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.36x | 1.46x | 5.49x | 1.38x | 1.43x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 73.02x | 116.06x | 161.13x | 109.69x | 5/5 |
| chess960_materialize | 1.06x | 1.93x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.17x | 1.78x | 1.23x | 1.92x | 1.86x | 10/10 |
| corpus_materialize | 1.02x | 2.07x | 1.86x | 1.38x | 3.04x | 1.77x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.10x | 2.96x | 2.27x | 2.82x | 1.62x | 10/10 |
| replay_trusted_native | 1.02x | 1.10x | 2.25x | 2.41x | 3.51x | 3.20x | 10/10 |
| replay_checked_native | 0.93x | 2.44x | 1.07x | 5.75x | 6.17x | 7.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
