| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.90x | 1.66x | 1.66x | 2.55x | 2.06x | 30/30 |
| materialize | 1.08x | 1.82x | 1.47x | 1.00x | 2.14x | 1.34x | 26/30 |
| perft_bulk_d4 | 1.16x | 3.80x | 1.49x | 1.24x | 1.89x | 1.78x | 29/30 |
| perft_full_d3 | 1.20x | 1.56x | 1.71x | 1.29x | 1.85x | 2.01x | 30/30 |
| fen_parse | 1.33x | 1.37x | 1.45x | 1.44x | 1.80x | 1.52x | 30/30 |
| fen_write_native_ep | 1.21x | 4.24x | 8.84x | 1.79x | 28.01x | 2.33x | 30/30 |
| snapshot | 1.15x | 1.70x | 1.13x | 2.50x | 1.15x | 1.04x | 25/30 |
| san_parse_all | 1.14x | 1.76x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.75x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.09x | 1.15x | 1.21x | 5.72x | 1.17x | 1.35x | 29/30 |
| make_unmake_full | 1.13x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.59x | 1.20x | 70.13x | 136.39x | 164.44x | 149.80x | 5/5 |
| chess960_materialize | 1.07x | 1.71x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 3.12x | 1.64x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.85x | 34.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.89x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 21.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.26x | 1.96x | 1.26x | 2.26x | 2.37x | 9/10 |
| corpus_materialize | 0.91x | 1.46x | 1.43x | 1.14x | 1.84x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.89x | 1.68x | 2.43x | 1.52x | 2.16x | 1.34x | 10/10 |
| replay_trusted_native | 0.98x | 0.72x | 1.71x | 1.76x | 2.19x | 1.93x | 8/10 |
| replay_checked_native | 0.96x | 2.48x | 1.10x | 8.33x | 7.54x | 9.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
