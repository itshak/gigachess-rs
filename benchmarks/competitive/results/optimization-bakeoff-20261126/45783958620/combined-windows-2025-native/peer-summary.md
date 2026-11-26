| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.37x | 1.57x | 1.53x | 2.39x | 1.93x | 30/30 |
| materialize | 0.98x | 1.90x | 1.29x | 1.09x | 2.16x | 1.38x | 29/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.55x | 1.38x | 2.38x | 2.12x | 30/30 |
| perft_full_d3 | 0.89x | 1.10x | 1.14x | 0.96x | 1.60x | 1.42x | 24/30 |
| fen_parse | 0.95x | 1.23x | 1.31x | 1.23x | 1.87x | 1.34x | 30/30 |
| fen_write_native_ep | 0.99x | 4.08x | 7.79x | 1.67x | 32.15x | 1.99x | 30/30 |
| snapshot | 0.32x | 1.17x | 1.07x | 0.87x | 1.08x | 0.97x | 17/30 |
| san_parse_all | 0.99x | 1.98x | N/A | 4.63x | N/A | N/A | 12/12 |
| san_render_native | 1.11x | 10.11x | N/A | 3.36x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.39x | 0.96x | 5.87x | 1.09x | 1.30x | 25/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.00x | 2.74x | 81.07x | 139.91x | 215.08x | 175.23x | 5/5 |
| chess960_materialize | 0.94x | 1.44x | 1.15x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.58x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 17.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.45x | 1.88x | 1.54x | 2.47x | 2.27x | 10/10 |
| corpus_materialize | 0.95x | 1.85x | 1.31x | 1.12x | 2.66x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.91x | 2.41x | 1.94x | 2.31x | 1.53x | 10/10 |
| replay_trusted_native | 0.88x | 0.65x | 1.17x | 1.39x | 2.24x | 2.88x | 8/10 |
| replay_checked_native | 0.97x | 2.06x | 0.92x | 5.09x | 6.44x | 6.96x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
