| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.52x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.77x | 1.53x | 1.40x | 1.96x | 2.01x | 30/30 |
| perft_full_d3 | 0.99x | 0.91x | 0.99x | 0.83x | 1.21x | 1.15x | 17/30 |
| fen_parse | 1.01x | 1.22x | 1.31x | 1.27x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.36x | 1.56x | 29.82x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.03x | 1.47x | 1.02x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.45x | N/A | 2.46x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.21x | 1.43x | 3.89x | 1.16x | 1.16x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.80x | 130.24x | 169.30x | 136.50x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.92x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.45x | 1.69x | 1.35x | 2.02x | 2.19x | 10/10 |
| corpus_materialize | 1.01x | 1.63x | 1.54x | 1.23x | 1.95x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.05x | 0.64x | 1.73x | 2.08x | 2.31x | 2.35x | 8/10 |
| replay_checked_native | 1.00x | 2.15x | 1.05x | 7.02x | 6.35x | 8.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
