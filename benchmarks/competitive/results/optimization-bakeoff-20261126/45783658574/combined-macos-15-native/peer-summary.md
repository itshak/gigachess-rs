| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.48x | 1.46x | 1.53x | 2.03x | 1.75x | 30/30 |
| materialize | 0.99x | 2.09x | 1.50x | 1.02x | 2.28x | 1.28x | 28/30 |
| perft_bulk_d4 | 1.00x | 4.57x | 1.74x | 1.41x | 2.14x | 2.01x | 30/30 |
| perft_full_d3 | 0.95x | 1.55x | 1.77x | 1.37x | 2.03x | 2.07x | 30/30 |
| fen_parse | 0.96x | 1.28x | 1.37x | 1.35x | 1.67x | 1.44x | 30/30 |
| fen_write_native_ep | 0.99x | 3.16x | 6.74x | 1.32x | 20.89x | 1.71x | 30/30 |
| snapshot | 0.98x | 1.75x | 1.15x | 2.45x | 1.07x | 1.06x | 26/30 |
| san_parse_all | 0.88x | 1.80x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.73x | N/A | 2.58x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 0.98x | 1.06x | 4.84x | 1.00x | 1.10x | 17/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.78x | 1.15x | 62.77x | 100.83x | 135.93x | 94.51x | 5/5 |
| chess960_materialize | 0.98x | 1.60x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.22x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 41.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.13x | 20.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 6.14x | 2.14x | 1.24x | 2.44x | 2.52x | 10/10 |
| corpus_materialize | 1.17x | 1.45x | 1.42x | 1.17x | 1.93x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.87x | 2.56x | 1.84x | 2.45x | 1.37x | 10/10 |
| replay_trusted_native | 1.05x | 0.57x | 1.52x | 1.57x | 1.72x | 1.63x | 8/10 |
| replay_checked_native | 1.02x | 2.10x | 0.95x | 7.39x | 6.60x | 7.94x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
