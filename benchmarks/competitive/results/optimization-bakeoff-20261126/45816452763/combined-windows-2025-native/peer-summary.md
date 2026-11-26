| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.26x | 1.53x | 1.46x | 2.13x | 1.85x | 30/30 |
| materialize | 1.00x | 1.91x | 1.31x | 1.19x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.69x | 1.41x | 1.23x | 2.03x | 1.81x | 30/30 |
| perft_full_d3 | 0.83x | 1.13x | 1.18x | 1.06x | 1.57x | 1.51x | 28/30 |
| fen_parse | 1.04x | 1.28x | 1.51x | 1.39x | 2.33x | 1.58x | 30/30 |
| fen_write_native_ep | 1.04x | 4.09x | 7.97x | 1.60x | 32.06x | 2.01x | 30/30 |
| snapshot | 1.02x | 2.08x | 1.03x | 4.12x | 1.07x | 1.05x | 29/30 |
| san_parse_all | 1.05x | 2.09x | N/A | 5.53x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 10.11x | N/A | 3.49x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.77x | 1.06x | 7.38x | 1.15x | 1.27x | 28/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.13x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 80.24x | 156.17x | 242.54x | 178.50x | 5/5 |
| chess960_materialize | 0.97x | 1.43x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.57x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 26.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 21.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.68x | 1.53x | 1.16x | 2.10x | 1.98x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.17x | 1.07x | 1.93x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.76x | 2.25x | 1.87x | 2.15x | 1.43x | 10/10 |
| replay_trusted_native | 0.92x | 0.65x | 1.24x | 1.65x | 2.05x | 2.31x | 8/10 |
| replay_checked_native | 0.95x | 2.34x | 0.92x | 6.97x | 7.01x | 8.50x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
