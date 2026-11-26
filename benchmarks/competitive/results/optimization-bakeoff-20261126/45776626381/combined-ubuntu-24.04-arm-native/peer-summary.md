| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.84x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.03x | 1.82x | 1.44x | 1.25x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.75x | 1.52x | 1.38x | 1.94x | 2.00x | 30/30 |
| perft_full_d3 | 1.42x | 1.17x | 1.27x | 1.07x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.28x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 3.05x | 6.27x | 1.06x | 19.84x | 1.59x | 29/30 |
| snapshot | 0.98x | 1.25x | 0.94x | 1.59x | 0.89x | 1.02x | 18/30 |
| san_parse_all | 1.04x | 1.85x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 11.86x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.23x | 1.11x | 1.28x | 3.61x | 1.07x | 1.10x | 28/30 |
| make_unmake_full | 1.25x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 77.16x | 129.61x | 174.20x | 139.92x | 5/5 |
| chess960_materialize | 1.07x | 1.51x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.08x | 2.62x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.76x | 1.85x | 1.47x | 2.16x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.57x | 1.50x | 1.19x | 1.88x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.50x | 2.11x | 2.94x | 2.09x | 2.79x | 1.46x | 10/10 |
| replay_trusted_native | 1.22x | 0.52x | 1.32x | 1.61x | 1.77x | 1.83x | 8/10 |
| replay_checked_native | 1.10x | 2.01x | 1.00x | 6.63x | 5.89x | 7.64x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
