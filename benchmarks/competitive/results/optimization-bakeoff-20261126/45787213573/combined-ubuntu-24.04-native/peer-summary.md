| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.02x | 1.73x | 1.45x | 2.28x | 1.76x | 30/30 |
| materialize | 1.00x | 2.10x | 1.58x | 1.62x | 2.44x | 1.82x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.89x | 1.59x | 1.21x | 2.40x | 1.91x | 30/30 |
| perft_full_d3 | 0.96x | 1.50x | 1.68x | 1.60x | 2.13x | 1.80x | 30/30 |
| fen_parse | 0.94x | 1.29x | 1.46x | 0.97x | 1.36x | 1.06x | 24/30 |
| fen_write_native_ep | 1.01x | 4.49x | 10.16x | 2.16x | 27.90x | 2.68x | 30/30 |
| snapshot | 0.70x | 1.57x | 1.24x | 2.81x | 1.25x | 1.14x | 30/30 |
| san_parse_all | 0.93x | 1.83x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.22x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.63x | 1.05x | 3.06x | 1.06x | 1.29x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.06x | N/A | N/A | 4/6 |
| startpos | 1.02x | 1.29x | 80.54x | 139.84x | 182.40x | 147.39x | 5/5 |
| chess960_materialize | 0.92x | 1.52x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.27x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 21.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.52x | 2.44x | 1.33x | 2.96x | 2.68x | 10/10 |
| corpus_materialize | 1.00x | 1.45x | 1.33x | 1.09x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.26x | 2.90x | 2.21x | 2.86x | 1.57x | 10/10 |
| replay_trusted_native | 0.93x | 0.67x | 1.22x | 1.17x | 1.68x | 1.93x | 7/10 |
| replay_checked_native | 0.97x | 2.00x | 1.12x | 8.37x | 8.28x | 9.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
