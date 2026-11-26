| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.36x | 1.37x | 1.44x | 1.91x | 1.68x | 30/30 |
| materialize | 1.03x | 2.04x | 1.46x | 1.09x | 2.55x | 1.39x | 27/30 |
| perft_bulk_d4 | 0.97x | 4.28x | 1.54x | 1.26x | 1.91x | 1.90x | 30/30 |
| perft_full_d3 | 1.02x | 0.92x | 1.01x | 0.77x | 1.12x | 1.17x | 15/30 |
| fen_parse | 1.01x | 1.30x | 1.39x | 1.44x | 1.68x | 1.53x | 30/30 |
| fen_write_native_ep | 0.99x | 2.95x | 6.47x | 1.27x | 20.02x | 1.71x | 30/30 |
| snapshot | 0.98x | 1.57x | 1.16x | 2.27x | 1.07x | 0.94x | 24/30 |
| san_parse_all | 1.11x | 1.66x | N/A | 3.45x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.56x | N/A | 2.59x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 0.80x | 0.83x | 3.97x | 0.83x | 0.88x | 7/30 |
| make_unmake_full | 1.18x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 1.59x | 1.21x | 69.70x | 123.16x | 154.57x | 126.77x | 5/5 |
| chess960_materialize | 1.06x | 1.43x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.94x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 37.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 19.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.27x | 2.06x | 1.28x | 2.41x | 2.95x | 10/10 |
| corpus_materialize | 1.02x | 1.46x | 1.38x | 1.10x | 1.87x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.32x | 1.92x | 1.41x | 1.88x | 1.04x | 10/10 |
| replay_trusted_native | 1.02x | 0.44x | 1.13x | 1.17x | 1.31x | 1.22x | 8/10 |
| replay_checked_native | 1.01x | 2.11x | 0.93x | 7.03x | 6.19x | 7.69x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
