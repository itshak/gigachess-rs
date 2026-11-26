| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.19x | 1.49x | 1.45x | 2.18x | 1.78x | 30/30 |
| materialize | 0.99x | 1.89x | 1.62x | 1.20x | 2.15x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.81x | 1.52x | 1.36x | 2.29x | 1.89x | 30/30 |
| perft_full_d3 | 0.99x | 1.27x | 1.43x | 1.26x | 1.82x | 1.84x | 30/30 |
| fen_parse | 1.05x | 1.40x | 1.37x | 1.22x | 2.20x | 1.26x | 30/30 |
| fen_write_native_ep | 0.98x | 4.39x | 9.36x | 1.70x | 27.22x | 2.12x | 30/30 |
| snapshot | 0.74x | 1.14x | 1.12x | 1.83x | 1.12x | 0.84x | 24/30 |
| san_parse_all | 0.96x | 2.21x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.56x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.02x | 1.17x | 4.04x | 1.30x | 1.44x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.14x | 66.81x | 128.38x | 191.89x | 125.16x | 5/5 |
| chess960_materialize | 1.00x | 1.49x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.44x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 24.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 19.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.36x | 1.77x | 1.37x | 2.53x | 2.35x | 10/10 |
| corpus_materialize | 1.01x | 1.65x | 1.59x | 1.22x | 2.09x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.40x | 3.15x | 2.39x | 3.01x | 1.69x | 10/10 |
| replay_trusted_native | 1.03x | 0.72x | 1.34x | 1.63x | 2.16x | 2.51x | 8/10 |
| replay_checked_native | 0.99x | 2.24x | 1.01x | 7.76x | 7.40x | 9.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
