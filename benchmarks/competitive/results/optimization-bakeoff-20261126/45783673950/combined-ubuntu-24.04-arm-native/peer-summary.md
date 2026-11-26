| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.01x | 1.83x | 1.45x | 1.26x | 2.01x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.52x | 1.37x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 0.95x | 1.10x | 1.20x | 1.00x | 1.46x | 1.39x | 24/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.29x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 1.01x | 2.98x | 6.15x | 1.06x | 19.73x | 1.58x | 29/30 |
| snapshot | 1.11x | 1.21x | 1.05x | 1.60x | 1.05x | 1.02x | 30/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.80x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.11x | 1.31x | 3.63x | 1.07x | 1.07x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.17x | 71.30x | 122.54x | 158.63x | 126.04x | 5/5 |
| chess960_materialize | 1.01x | 1.51x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.78x | 1.83x | 1.47x | 2.17x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.51x | 1.21x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.09x | 2.80x | 1.46x | 10/10 |
| replay_trusted_native | 0.97x | 0.49x | 1.28x | 1.55x | 1.72x | 1.77x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 0.99x | 6.70x | 5.94x | 7.75x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
