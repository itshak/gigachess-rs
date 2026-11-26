| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 2.67x | 1.51x | 1.31x | 1.77x | 1.27x | 29/30 |
| materialize | 0.95x | 2.28x | 1.75x | 1.74x | 2.69x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.98x | 2.95x | 1.56x | 1.16x | 1.96x | 1.75x | 29/30 |
| perft_full_d3 | 1.05x | 1.42x | 1.69x | 1.47x | 2.06x | 1.70x | 30/30 |
| fen_parse | 1.12x | 1.50x | 1.52x | 1.23x | 1.58x | 1.20x | 30/30 |
| fen_write_native_ep | 0.93x | 2.91x | 9.13x | 1.67x | 24.44x | 2.34x | 30/30 |
| snapshot | 0.89x | 1.46x | 1.12x | 3.02x | 1.12x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.89x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.52x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.17x | 1.42x | 4.11x | 1.34x | 1.53x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.17x | 1.50x | 66.82x | 129.86x | 147.80x | 111.82x | 5/5 |
| chess960_materialize | 0.85x | 1.53x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.04x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 41.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.11x | 23.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 4.45x | 2.23x | 1.22x | 2.28x | 3.07x | 10/10 |
| corpus_materialize | 0.98x | 1.48x | 1.37x | 1.12x | 1.80x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 2.93x | 2.17x | 2.87x | 1.55x | 10/10 |
| replay_trusted_native | 0.91x | 0.78x | 1.48x | 1.53x | 2.03x | 1.85x | 8/10 |
| replay_checked_native | 0.89x | 1.85x | 1.00x | 7.90x | 7.78x | 8.80x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
