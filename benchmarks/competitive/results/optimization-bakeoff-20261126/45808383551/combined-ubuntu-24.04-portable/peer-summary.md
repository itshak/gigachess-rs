| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.74x | 1.56x | 1.35x | 1.81x | 1.29x | 29/30 |
| materialize | 0.99x | 2.27x | 1.76x | 1.78x | 2.75x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.91x | 2.68x | 1.43x | 1.06x | 1.77x | 1.59x | 29/30 |
| perft_full_d3 | 0.97x | 1.52x | 1.77x | 1.57x | 2.10x | 1.79x | 30/30 |
| fen_parse | 1.02x | 1.60x | 1.47x | 1.17x | 1.51x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 3.05x | 10.44x | 1.89x | 28.53x | 2.28x | 30/30 |
| snapshot | 1.12x | 1.75x | 1.25x | 3.24x | 1.25x | 1.00x | 25/30 |
| san_parse_all | 1.01x | 1.95x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.35x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.01x | 1.31x | 3.80x | 1.28x | 1.44x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.35x | 56.97x | 105.03x | 123.65x | 98.61x | 5/5 |
| chess960_materialize | 1.02x | 1.70x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 1.98x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 42.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 24.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.77x | 1.90x | 1.04x | 1.99x | 2.58x | 10/10 |
| corpus_materialize | 0.93x | 1.44x | 1.33x | 1.09x | 1.79x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.92x | 3.01x | 2.16x | 2.96x | 1.52x | 10/10 |
| replay_trusted_native | 0.98x | 0.77x | 1.52x | 1.51x | 2.01x | 1.86x | 8/10 |
| replay_checked_native | 1.00x | 2.00x | 1.13x | 8.57x | 8.45x | 9.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
