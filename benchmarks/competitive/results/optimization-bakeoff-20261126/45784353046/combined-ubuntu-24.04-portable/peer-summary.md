| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.93x | 1.48x | 1.35x | 1.88x | 1.33x | 30/30 |
| materialize | 0.99x | 1.99x | 1.76x | 1.26x | 2.34x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.89x | 1.57x | 1.23x | 1.96x | 1.69x | 30/30 |
| perft_full_d3 | 1.00x | 1.41x | 1.63x | 1.28x | 1.93x | 1.77x | 30/30 |
| fen_parse | 0.94x | 1.31x | 1.34x | 1.13x | 2.02x | 1.22x | 30/30 |
| fen_write_native_ep | 1.02x | 2.95x | 9.91x | 1.68x | 27.63x | 2.01x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.20x | 3.03x | 1.02x | 1.07x | 30/30 |
| san_parse_all | 0.97x | 1.97x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.87x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.33x | 3.85x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 64.46x | 107.33x | 173.13x | 114.24x | 5/5 |
| chess960_materialize | 1.00x | 1.56x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.07x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 23.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.03x | 18.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.92x | 1.94x | 1.30x | 2.14x | 2.58x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.63x | 1.18x | 2.10x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.15x | 2.22x | 3.29x | 2.49x | 3.17x | 1.84x | 10/10 |
| replay_trusted_native | 0.98x | 0.69x | 1.37x | 1.56x | 2.07x | 1.92x | 8/10 |
| replay_checked_native | 1.01x | 2.13x | 1.07x | 7.38x | 7.36x | 8.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
