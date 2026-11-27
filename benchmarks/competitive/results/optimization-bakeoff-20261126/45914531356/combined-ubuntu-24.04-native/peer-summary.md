| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.63x | 1.67x | 1.57x | 2.50x | 1.96x | 30/30 |
| materialize | 1.06x | 1.95x | 1.71x | 1.27x | 2.37x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.01x | 1.60x | 1.38x | 2.40x | 2.01x | 30/30 |
| perft_full_d3 | 1.00x | 1.58x | 1.55x | 1.32x | 1.91x | 1.86x | 30/30 |
| fen_parse | 1.14x | 1.30x | 1.39x | 1.26x | 2.19x | 1.39x | 30/30 |
| fen_write_native_ep | 0.97x | 3.97x | 8.91x | 1.73x | 27.07x | 1.97x | 30/30 |
| snapshot | 0.99x | 1.93x | 1.22x | 3.21x | 1.22x | 1.21x | 30/30 |
| san_parse_all | 1.00x | 2.03x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.42x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.33x | 1.34x | 4.62x | 1.48x | 1.62x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.25x | 79.43x | 141.46x | 229.45x | 155.78x | 5/5 |
| chess960_materialize | 0.99x | 1.62x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.78x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.89x | 28.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.91x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.18x | 1.75x | 1.32x | 2.54x | 2.34x | 10/10 |
| corpus_materialize | 0.93x | 1.56x | 1.50x | 1.13x | 2.03x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.34x | 3.15x | 2.41x | 3.01x | 1.75x | 10/10 |
| replay_trusted_native | 0.97x | 0.93x | 1.65x | 2.00x | 2.61x | 2.98x | 9/10 |
| replay_checked_native | 0.98x | 2.46x | 1.12x | 8.16x | 8.48x | 9.54x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
