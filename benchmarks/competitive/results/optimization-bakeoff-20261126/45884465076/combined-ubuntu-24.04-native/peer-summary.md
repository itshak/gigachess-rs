| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.29x | 1.84x | 1.56x | 2.46x | 1.90x | 30/30 |
| materialize | 1.01x | 2.15x | 1.66x | 1.67x | 2.56x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.73x | 1.53x | 1.16x | 2.30x | 1.85x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.54x | 1.53x | 2.01x | 1.72x | 30/30 |
| fen_parse | 0.93x | 1.31x | 1.29x | 1.09x | 1.51x | 1.07x | 30/30 |
| fen_write_native_ep | 1.00x | 5.03x | 10.57x | 2.12x | 30.17x | 2.54x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 25/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.96x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.36x | 4.10x | 1.43x | 1.64x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.80x | 0.99x | 77.47x | 147.47x | 188.88x | 142.54x | 4/5 |
| chess960_materialize | 0.99x | 1.77x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.55x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 38.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.98x | 1.90x | 1.05x | 2.26x | 2.11x | 10/10 |
| corpus_materialize | 1.01x | 1.39x | 1.29x | 1.07x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.32x | 3.13x | 2.25x | 3.05x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.88x | 1.48x | 1.54x | 2.16x | 2.45x | 9/10 |
| replay_checked_native | 1.01x | 2.23x | 1.13x | 9.24x | 8.96x | 10.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
