| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.20x | 1.54x | 1.49x | 2.34x | 1.79x | 30/30 |
| materialize | 0.97x | 1.95x | 1.58x | 1.12x | 2.01x | 1.41x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.90x | 1.52x | 1.23x | 2.36x | 2.05x | 30/30 |
| perft_full_d3 | 1.03x | 1.29x | 1.33x | 1.01x | 1.83x | 1.57x | 27/30 |
| fen_parse | 1.11x | 1.49x | 1.63x | 1.25x | 2.32x | 1.27x | 30/30 |
| fen_write_native_ep | 0.98x | 3.94x | 9.66x | 1.86x | 26.99x | 1.99x | 30/30 |
| snapshot | 1.23x | 1.90x | 1.25x | 3.24x | 1.36x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.22x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.95x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.59x | 1.06x | 3.70x | 1.15x | 1.39x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.20x | 93.58x | 142.29x | 230.75x | 142.34x | 5/5 |
| chess960_materialize | 0.94x | 1.60x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.75x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 17.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.48x | 1.81x | 1.33x | 2.41x | 2.09x | 10/10 |
| corpus_materialize | 1.01x | 1.88x | 1.76x | 1.24x | 2.81x | 1.53x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.29x | 3.10x | 2.31x | 2.95x | 1.65x | 10/10 |
| replay_trusted_native | 1.00x | 0.74x | 1.37x | 1.47x | 2.89x | 3.02x | 8/10 |
| replay_checked_native | 1.01x | 2.52x | 1.01x | 5.50x | 6.86x | 7.35x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
