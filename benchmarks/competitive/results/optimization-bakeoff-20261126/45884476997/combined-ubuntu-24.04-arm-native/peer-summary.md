| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.68x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.52x | 1.34x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.57x | 1.43x | 2.02x | 2.05x | 30/30 |
| perft_full_d3 | 1.12x | 1.66x | 1.81x | 1.50x | 2.20x | 2.08x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.34x | 1.28x | 1.76x | 1.39x | 30/30 |
| fen_write_native_ep | 0.99x | 4.59x | 9.37x | 1.57x | 29.98x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.11x | 1.03x | 1.47x | 1.05x | 0.95x | 23/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 12.78x | N/A | 2.76x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.56x | 129.19x | 171.53x | 136.24x | 5/5 |
| chess960_materialize | 0.99x | 1.69x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.14x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.47x | 1.71x | 1.34x | 1.99x | 2.21x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.53x | 1.23x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.13x | 2.92x | 2.09x | 2.84x | 1.46x | 10/10 |
| replay_trusted_native | 0.91x | 0.58x | 1.51x | 1.81x | 2.05x | 2.10x | 8/10 |
| replay_checked_native | 0.97x | 2.24x | 1.11x | 7.43x | 6.53x | 8.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
