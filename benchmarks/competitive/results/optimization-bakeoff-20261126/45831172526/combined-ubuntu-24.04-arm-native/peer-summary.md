| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.94x | 1.49x | 1.63x | 1.93x | 1.93x | 30/30 |
| materialize | 1.00x | 1.85x | 1.44x | 1.26x | 2.03x | 1.41x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.43x | 1.39x | 1.26x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.05x | 1.54x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.22x | 1.31x | 1.25x | 1.74x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.37x | 9.25x | 1.57x | 30.47x | 2.45x | 30/30 |
| snapshot | 0.98x | 1.05x | 1.08x | 1.45x | 1.09x | 0.95x | 24/30 |
| san_parse_all | 0.98x | 1.81x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.88x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 4.02x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.58x | 124.82x | 173.54x | 138.12x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.82x | 1.47x | 1.17x | 1.71x | 1.88x | 10/10 |
| corpus_materialize | 0.99x | 1.51x | 1.44x | 1.15x | 1.80x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.09x | 2.90x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.02x | 0.64x | 1.64x | 1.99x | 2.24x | 2.26x | 8/10 |
| replay_checked_native | 1.01x | 2.16x | 1.05x | 7.07x | 6.33x | 8.20x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
