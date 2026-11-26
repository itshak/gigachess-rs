| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.57x | 1.57x | 1.51x | 2.34x | 1.83x | 30/30 |
| materialize | 0.92x | 2.11x | 1.65x | 1.14x | 2.08x | 1.48x | 29/30 |
| perft_bulk_d4 | 0.93x | 3.96x | 1.51x | 1.23x | 2.36x | 2.04x | 30/30 |
| perft_full_d3 | 0.86x | 1.33x | 1.37x | 1.02x | 1.88x | 1.61x | 27/30 |
| fen_parse | 0.94x | 1.40x | 1.50x | 1.22x | 1.88x | 1.30x | 30/30 |
| fen_write_native_ep | 0.89x | 3.73x | 8.26x | 1.66x | 24.54x | 2.13x | 30/30 |
| snapshot | 0.89x | 1.87x | 1.33x | 3.19x | 1.33x | 1.22x | 30/30 |
| san_parse_all | 0.93x | 2.09x | N/A | 3.61x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.92x | N/A | 2.02x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.97x | 1.32x | 4.87x | 1.47x | 1.75x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.50x | 108.97x | 176.69x | 252.40x | 186.52x | 5/5 |
| chess960_materialize | 0.89x | 1.73x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.83x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 21.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 18.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.15x | 1.64x | 1.21x | 2.19x | 1.91x | 9/10 |
| corpus_materialize | 0.97x | 1.86x | 1.70x | 1.18x | 2.75x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.95x | 2.27x | 3.03x | 2.25x | 2.82x | 1.61x | 10/10 |
| replay_trusted_native | 1.08x | 1.19x | 2.02x | 2.34x | 4.00x | 5.07x | 10/10 |
| replay_checked_native | 0.88x | 2.62x | 0.94x | 5.44x | 6.22x | 7.50x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
