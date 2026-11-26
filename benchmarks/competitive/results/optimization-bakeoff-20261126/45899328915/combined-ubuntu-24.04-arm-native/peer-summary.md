| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.06x | 1.55x | 1.67x | 2.01x | 2.01x | 30/30 |
| materialize | 0.99x | 1.92x | 1.52x | 1.33x | 2.13x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.76x | 1.54x | 1.39x | 1.97x | 2.01x | 30/30 |
| perft_full_d3 | 1.02x | 1.50x | 1.63x | 1.37x | 1.98x | 1.89x | 30/30 |
| fen_parse | 0.98x | 1.22x | 1.30x | 1.24x | 1.72x | 1.36x | 30/30 |
| fen_write_native_ep | 0.95x | 4.33x | 8.78x | 1.49x | 28.27x | 2.27x | 30/30 |
| snapshot | 1.09x | 1.25x | 1.02x | 1.58x | 0.99x | 1.01x | 26/30 |
| san_parse_all | 0.98x | 1.80x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 12.18x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.26x | 1.47x | 4.14x | 1.21x | 1.22x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.00x | 62.09x | 104.09x | 143.64x | 115.95x | 4/5 |
| chess960_materialize | 0.99x | 1.69x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.77x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 26.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.54x | 1.77x | 1.41x | 2.05x | 2.27x | 10/10 |
| corpus_materialize | 0.98x | 1.56x | 1.50x | 1.20x | 1.86x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.13x | 2.92x | 2.10x | 2.83x | 1.46x | 10/10 |
| replay_trusted_native | 0.94x | 0.60x | 1.56x | 1.91x | 2.09x | 2.14x | 8/10 |
| replay_checked_native | 0.95x | 2.18x | 1.07x | 7.21x | 6.44x | 8.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
