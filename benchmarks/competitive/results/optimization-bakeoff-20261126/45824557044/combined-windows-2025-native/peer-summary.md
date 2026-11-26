| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.55x | 1.50x | 1.42x | 2.21x | 1.86x | 30/30 |
| materialize | 0.97x | 2.06x | 1.37x | 1.20x | 2.31x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.08x | 1.40x | 1.19x | 2.17x | 1.98x | 30/30 |
| perft_full_d3 | 0.97x | 2.17x | 1.41x | 1.22x | 1.90x | 1.93x | 30/30 |
| fen_parse | 0.98x | 1.30x | 1.52x | 1.51x | 2.18x | 1.53x | 30/30 |
| fen_write_native_ep | 0.98x | 3.63x | 7.25x | 1.48x | 30.45x | 1.99x | 30/30 |
| snapshot | 0.73x | 1.91x | 1.12x | 3.34x | 1.11x | 1.11x | 30/30 |
| san_parse_all | 1.00x | 1.98x | N/A | 4.63x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 10.16x | N/A | 3.50x | N/A | N/A | 12/12 |
| apply_children_full | 0.67x | 1.22x | 0.90x | 5.85x | 1.06x | 1.27x | 23/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.25x | 88.57x | 171.97x | 235.64x | 182.45x | 5/5 |
| chess960_materialize | 0.90x | 1.49x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.71x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 38.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 26.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.16x | 1.53x | 1.01x | 1.98x | 1.88x | 9/10 |
| corpus_materialize | 0.99x | 1.61x | 1.23x | 1.00x | 2.16x | 1.31x | 9/10 |
| corpus_uci_sorted | 1.01x | 1.62x | 1.97x | 1.69x | 1.97x | 1.39x | 10/10 |
| replay_trusted_native | 1.03x | 0.70x | 1.39x | 1.63x | 2.52x | 3.14x | 8/10 |
| replay_checked_native | 1.02x | 2.37x | 1.15x | 8.47x | 8.45x | 11.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
