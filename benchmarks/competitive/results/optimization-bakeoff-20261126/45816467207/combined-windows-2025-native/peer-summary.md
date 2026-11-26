| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.22x | 1.50x | 1.44x | 2.12x | 1.84x | 30/30 |
| materialize | 1.02x | 1.88x | 1.30x | 1.19x | 2.20x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.47x | 1.39x | 1.17x | 1.99x | 1.77x | 30/30 |
| perft_full_d3 | 0.99x | 1.30x | 1.37x | 1.25x | 1.79x | 1.76x | 30/30 |
| fen_parse | 0.88x | 1.13x | 1.29x | 1.35x | 1.98x | 1.32x | 30/30 |
| fen_write_native_ep | 0.99x | 4.24x | 9.12x | 1.68x | 35.61x | 2.03x | 30/30 |
| snapshot | 0.96x | 1.87x | 1.00x | 3.85x | 0.99x | 1.15x | 23/30 |
| san_parse_all | 1.00x | 1.98x | N/A | 5.26x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.48x | N/A | 3.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.08x | 1.26x | 8.68x | 1.34x | 1.49x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.21x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 79.49x | 168.70x | 240.35x | 179.43x | 5/5 |
| chess960_materialize | 1.03x | 1.56x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 18.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.05x | 1.69x | 1.26x | 2.24x | 2.28x | 10/10 |
| corpus_materialize | 0.98x | 1.47x | 1.25x | 1.05x | 1.89x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.88x | 2.33x | 2.09x | 2.31x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.72x | 1.46x | 1.81x | 2.21x | 2.55x | 8/10 |
| replay_checked_native | 0.93x | 2.17x | 0.95x | 7.28x | 7.03x | 8.45x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
