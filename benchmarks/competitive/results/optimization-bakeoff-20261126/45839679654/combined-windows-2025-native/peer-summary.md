| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.62x | 1.64x | 1.63x | 2.28x | 2.00x | 30/30 |
| materialize | 0.97x | 2.00x | 1.35x | 1.20x | 2.27x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.98x | 1.56x | 1.32x | 2.29x | 2.00x | 30/30 |
| perft_full_d3 | 0.85x | 0.99x | 1.06x | 0.93x | 1.42x | 1.30x | 22/30 |
| fen_parse | 1.06x | 1.25x | 1.37x | 1.25x | 2.14x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.28x | 8.22x | 1.61x | 33.49x | 2.04x | 30/30 |
| snapshot | 0.99x | 2.02x | 1.05x | 4.17x | 1.08x | 1.11x | 27/30 |
| san_parse_all | 0.92x | 1.95x | N/A | 5.19x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 9.69x | N/A | 3.20x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 2.04x | 1.28x | 8.34x | 1.31x | 1.50x | 28/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.94x | 1.17x | 78.74x | 148.17x | 230.74x | 173.34x | 5/5 |
| chess960_materialize | 0.83x | 1.33x | 1.05x | N/A | N/A | N/A | 3/4 |
| chess960_perft_d3 | 0.89x | 2.99x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 26.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.93x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.03x | 21.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.36x | 1.90x | 1.34x | 2.43x | 2.42x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.23x | 1.15x | 1.98x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.78x | 2.24x | 1.82x | 2.51x | 1.43x | 10/10 |
| replay_trusted_native | 0.92x | 0.78x | 1.48x | 2.00x | 2.41x | 2.73x | 8/10 |
| replay_checked_native | 0.86x | 2.08x | 0.89x | 6.81x | 6.73x | 8.30x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
