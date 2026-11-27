| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.34x | 1.86x | 1.56x | 2.47x | 1.90x | 30/30 |
| materialize | 1.06x | 2.27x | 1.69x | 1.72x | 2.76x | 2.07x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.78x | 1.54x | 1.17x | 2.33x | 1.85x | 29/30 |
| perft_full_d3 | 1.02x | 1.36x | 1.58x | 1.51x | 2.02x | 1.68x | 30/30 |
| fen_parse | 0.98x | 1.38x | 1.31x | 1.06x | 1.47x | 1.25x | 29/30 |
| fen_write_native_ep | 1.00x | 5.00x | 10.26x | 2.12x | 29.06x | 2.69x | 30/30 |
| snapshot | 1.01x | 2.00x | 1.00x | 4.68x | 1.00x | 1.33x | 26/30 |
| san_parse_all | 0.90x | 1.71x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.93x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.18x | 1.37x | 4.08x | 1.45x | 1.67x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.00x | 78.98x | 137.09x | 169.99x | 153.78x | 5/5 |
| chess960_materialize | 0.99x | 1.85x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.47x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 37.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 19.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.45x | 2.05x | 1.13x | 2.49x | 2.24x | 10/10 |
| corpus_materialize | 1.01x | 1.40x | 1.30x | 1.06x | 1.72x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 3.08x | 2.22x | 3.00x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.89x | 1.57x | 1.53x | 2.18x | 2.56x | 9/10 |
| replay_checked_native | 0.95x | 2.08x | 1.13x | 8.57x | 8.27x | 10.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
