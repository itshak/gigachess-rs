| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.28x | 1.68x | 1.54x | 2.18x | 1.52x | 30/30 |
| materialize | 1.01x | 2.12x | 1.88x | 1.36x | 2.52x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.25x | 1.77x | 1.37x | 2.23x | 1.91x | 30/30 |
| perft_full_d3 | 1.18x | 1.60x | 1.88x | 1.52x | 2.24x | 2.06x | 30/30 |
| fen_parse | 0.95x | 1.54x | 1.48x | 1.17x | 2.22x | 1.32x | 30/30 |
| fen_write_native_ep | 1.04x | 3.22x | 9.84x | 1.73x | 27.86x | 1.97x | 30/30 |
| snapshot | 0.99x | 1.68x | 1.23x | 3.05x | 1.04x | 0.97x | 25/30 |
| san_parse_all | 1.00x | 2.04x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.37x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.23x | 1.57x | 4.51x | 1.45x | 1.49x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 66.59x | 106.82x | 180.19x | 115.30x | 5/5 |
| chess960_materialize | 0.98x | 1.70x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.39x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.26x | 2.10x | 1.41x | 2.30x | 2.81x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.61x | 1.18x | 2.09x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.12x | 3.13x | 2.36x | 3.03x | 1.66x | 10/10 |
| replay_trusted_native | 0.98x | 0.89x | 1.77x | 1.97x | 2.61x | 2.44x | 9/10 |
| replay_checked_native | 1.01x | 2.37x | 1.17x | 7.97x | 8.20x | 9.19x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
