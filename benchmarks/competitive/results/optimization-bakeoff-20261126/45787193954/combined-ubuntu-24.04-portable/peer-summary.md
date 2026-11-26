| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.66x | 1.51x | 1.32x | 1.74x | 1.27x | 29/30 |
| materialize | 1.00x | 2.19x | 1.77x | 1.82x | 2.65x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.96x | 1.57x | 1.17x | 1.94x | 1.75x | 30/30 |
| perft_full_d3 | 0.98x | 1.50x | 1.78x | 1.56x | 2.06x | 1.76x | 30/30 |
| fen_parse | 1.01x | 1.47x | 1.39x | 1.20x | 1.46x | 1.19x | 30/30 |
| fen_write_native_ep | 0.98x | 2.98x | 9.73x | 1.81x | 27.26x | 2.49x | 30/30 |
| snapshot | 1.12x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.98x | 1.89x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.36x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.84x | 1.18x | 3.45x | 1.13x | 1.29x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 61.87x | 112.14x | 123.77x | 102.23x | 5/5 |
| chess960_materialize | 1.04x | 1.52x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.88x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 43.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 18.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.66x | 2.31x | 1.27x | 2.37x | 3.18x | 10/10 |
| corpus_materialize | 0.99x | 1.49x | 1.41x | 1.13x | 1.83x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.98x | 3.01x | 2.20x | 2.93x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.70x | 1.35x | 1.36x | 1.79x | 1.66x | 8/10 |
| replay_checked_native | 1.00x | 1.91x | 1.11x | 8.41x | 8.22x | 9.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
