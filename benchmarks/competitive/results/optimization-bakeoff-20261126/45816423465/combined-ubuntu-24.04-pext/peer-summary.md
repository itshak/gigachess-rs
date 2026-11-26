| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.01x | 1.69x | 1.47x | 2.25x | 1.70x | 30/30 |
| materialize | 0.97x | 2.01x | 1.50x | 1.68x | 2.40x | 1.77x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.40x | 1.35x | 1.06x | 2.11x | 1.66x | 29/30 |
| perft_full_d3 | 0.98x | 1.46x | 1.73x | 1.66x | 2.12x | 1.78x | 30/30 |
| fen_parse | 0.94x | 1.34x | 1.28x | 1.08x | 1.40x | 1.08x | 30/30 |
| fen_write_native_ep | 1.01x | 4.65x | 10.23x | 2.13x | 28.54x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 7.00x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 2.13x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.14x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.88x | 1.19x | 3.66x | 1.19x | 1.50x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.12x | 93.91x | 167.36x | 196.13x | 162.95x | 5/5 |
| chess960_materialize | 1.04x | 1.56x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.35x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 38.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.85x | 16.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.49x | 1.65x | 0.93x | 2.05x | 1.84x | 8/10 |
| corpus_materialize | 0.96x | 1.37x | 1.27x | 1.07x | 1.69x | 1.14x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.26x | 2.94x | 2.23x | 2.85x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.84x | 1.51x | 1.46x | 2.04x | 2.33x | 9/10 |
| replay_checked_native | 0.98x | 2.09x | 1.13x | 8.81x | 8.29x | 9.74x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
