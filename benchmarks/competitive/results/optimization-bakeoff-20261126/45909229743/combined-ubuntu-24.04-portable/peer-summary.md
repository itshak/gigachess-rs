| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.45x | 1.73x | 1.58x | 2.18x | 1.51x | 30/30 |
| materialize | 1.01x | 2.51x | 1.86x | 1.38x | 2.49x | 1.73x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.18x | 1.69x | 1.33x | 2.22x | 1.83x | 30/30 |
| perft_full_d3 | 1.24x | 1.77x | 2.07x | 1.60x | 2.44x | 2.28x | 30/30 |
| fen_parse | 0.93x | 1.60x | 1.49x | 1.17x | 1.90x | 1.29x | 30/30 |
| fen_write_native_ep | 0.99x | 2.65x | 9.15x | 1.75x | 26.09x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.22x | 2.86x | 1.13x | 0.98x | 24/30 |
| san_parse_all | 0.98x | 2.20x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.04x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.39x | 1.48x | 5.55x | 1.40x | 1.46x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 73.61x | 105.15x | 161.41x | 110.87x | 5/5 |
| chess960_materialize | 1.03x | 2.00x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.39x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 3.44x | 1.91x | 1.34x | 2.09x | 1.99x | 10/10 |
| corpus_materialize | 1.01x | 2.09x | 1.90x | 1.39x | 3.09x | 1.76x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.07x | 2.98x | 2.27x | 2.82x | 1.61x | 10/10 |
| replay_trusted_native | 0.99x | 1.08x | 2.17x | 2.36x | 3.47x | 3.15x | 10/10 |
| replay_checked_native | 1.00x | 2.63x | 1.15x | 6.20x | 7.09x | 7.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
