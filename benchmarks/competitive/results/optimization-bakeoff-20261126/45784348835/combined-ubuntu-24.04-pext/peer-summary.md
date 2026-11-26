| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.69x | 1.62x | 1.64x | 2.54x | 1.88x | 30/30 |
| materialize | 1.04x | 2.23x | 1.71x | 1.17x | 2.22x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.09x | 1.59x | 1.32x | 2.63x | 2.15x | 30/30 |
| perft_full_d3 | 1.02x | 1.26x | 1.30x | 0.99x | 2.04x | 1.46x | 26/30 |
| fen_parse | 0.98x | 1.31x | 1.49x | 1.10x | 1.92x | 1.13x | 30/30 |
| fen_write_native_ep | 1.00x | 3.97x | 9.73x | 1.84x | 27.87x | 2.11x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.21x | 2.82x | 1.26x | 1.11x | 30/30 |
| san_parse_all | 1.01x | 2.25x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.56x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.74x | 1.18x | 4.20x | 1.29x | 1.52x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.41x | 0.48x | 34.56x | 51.50x | 84.30x | 52.53x | 4/5 |
| chess960_materialize | 1.00x | 1.83x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.94x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.02x | 2.03x | 1.47x | 2.66x | 2.18x | 10/10 |
| corpus_materialize | 1.04x | 2.01x | 1.87x | 1.28x | 2.93x | 1.57x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.37x | 3.18x | 2.37x | 2.95x | 1.68x | 10/10 |
| replay_trusted_native | 1.17x | 0.81x | 1.30x | 1.54x | 2.77x | 3.22x | 9/10 |
| replay_checked_native | 1.05x | 2.50x | 0.95x | 5.45x | 6.57x | 7.20x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
